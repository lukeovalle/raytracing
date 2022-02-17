use image::{ImageBuffer, Rgb, Pixel};
use nalgebra::Vector3;
use indicatif::{ProgressBar, ProgressStyle};
use crate::camara::Cámara;
use crate::modelos::Modelo;
use crate::material::{Color, mezclar_colores};
use crate::geometria::{Punto, Rayo};

#[derive(Clone, Copy, Debug)]
pub struct Luz {
    fuente: Punto
}

impl Luz {
    pub fn new(fuente: &Punto) -> Luz {
        Luz { fuente: fuente.clone() }
    }

    fn fuente(&self) -> Punto {
        self.fuente
    }

    // si hay un obstáculo devuelve 0, si no devuelve la atenuación
    fn atenuación_con_sombra(&self, punto: &Punto, obstáculo: bool) -> f64 {
        match obstáculo {
            true => { 0.0 }
            false => { self.atenuación(punto) }
        }
    }


    // atenuación del 0 al 1 según la distancia
    fn atenuación(&self, punto: &Punto) -> f64 {
        1.0 / (self.fuente - punto).norm().sqrt()
    }
}

pub struct Escena<'a> {
    cámara: Cámara,
    objetos: Vec<&'a dyn Modelo>,
    luces: Vec<Luz>
}

impl<'a> Escena<'a> {
    pub fn new(cámara: &Cámara) -> Escena {
        Escena {
            cámara: cámara.clone(),
            objetos: Vec::new(),
            luces: Vec::new()
        }
    }

    pub fn añadir_objeto(&mut self, objeto: &'a (dyn Modelo + 'a)) -> Result<(), anyhow::Error> {
        self.objetos.push(objeto);
        Ok(())
    }

    pub fn añadir_luz(&mut self, luz: &Luz) -> Result<(), anyhow::Error> {
        self.luces.push(luz.clone());
        Ok(())
    }

    pub fn renderizar(&self) -> Result<ImageBuffer<Rgb<u8>, Vec<<Rgb<u8> as Pixel>::Subpixel>>, anyhow::Error> 
    {
        let mut buffer_img = ImageBuffer::new(self.cámara.ancho(), self.cámara.alto());

        // barrita de carga
        let barrita = ProgressBar::new(self.cámara.ancho() as u64 * self.cámara.alto() as u64);

        barrita.set_style(ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise} ({duration} estimado)] [{wide_bar:.cyan/blue}] {percent}%")
            .progress_chars("#>-")
        );

        for (x, y, pixel) in buffer_img.enumerate_pixels_mut() {
            // Integración de Monte Carlo
            let muestras_por_pixel = 100;
            let colores: Vec<Color> = (0..muestras_por_pixel).map(|_| {
                let (v_1, v_2): (f64, f64) = (rand::random(), rand::random());
                let rayo = self.cámara.lanzar_rayo(x as f64 + v_1 - 0.5 , y as f64 + v_2 - 0.5);
                
                self.trazar_rayo(&rayo, 20)
            }).collect();

            let mut color = mezclar_colores(&colores);

            color.x = color.x.clamp(0.0, 1.0 - 1e-10);
            color.y = color.y.clamp(0.0, 1.0 - 1e-10);
            color.z = color.z.clamp(0.0, 1.0 - 1e-10);

            // corrección gamma
            *pixel = Rgb([
                (256.0 * color.x.powf(1.0 / 2.2)) as u8,
                (256.0 * color.y.powf(1.0 / 2.2)) as u8,
                (256.0 * color.z.powf(1.0 / 2.2)) as u8
            ]);

            barrita.set_position(y as u64 * self.cámara.ancho() as u64 + (x + 1) as u64);
        }

        barrita.finish_with_message("ARchivo guardado en archivo.bmp");

        Ok(buffer_img)
    }

    fn trazar_rayo(&self, rayo: &Rayo, iteraciones: usize ) -> Color {
        if iteraciones == 0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        match self.intersecar_rayo(rayo) {
            Some((objeto, t)) => {
                let punto = rayo.evaluar(t);
                let normal_saliente = objeto.normal(&punto);
                let normal: Vector3<f64>;

                if normal_saliente.dot(&rayo.dirección()) > 0.0 {
                    // la normal apunta para "adentro" del objeto
                    normal = -normal_saliente;
                } else {
                    // apunta para afuera
                    normal = normal_saliente;
                }

                // devuelvo el color en el punto
                self.sombrear_punto(objeto, &punto, &normal, iteraciones)
            }
            None => { Color::new(0.0, 0.0, 0.0) }
        }
    }

    fn sombrear_punto(
        &self,
        objeto: &'a (dyn Modelo + 'a),
        punto: &Punto,
        normal: &Vector3<f64>,
        iteraciones: usize
    ) -> Color {
        let mut color: Color = Color::zeros();

        if let Some(col) = objeto.material().color_emitido {
            color += col;
        }

        let dirección = crate::geometria::versor_aleatorio_densidad_cos(normal);
        let rayo = Rayo::new(&(punto + normal * 1e-10), &dirección);
        
        if let Some(col) = objeto.material().color_ambiente {
            color += self.trazar_rayo(&rayo, iteraciones - 1).component_mul(&col);
        }
        
        // por ahora saco los shadow rays
        /*
        for luz in &self.luces {
            let dirección = luz.fuente() - punto;
            // corro el origen del rayo para que no choque con el objeto que quiero sombrear
            let rayo = Rayo::new(&(punto + normal * 1e-10), &dirección);
            let obstáculo = self.intersecar_rayo(&rayo);

            if obstáculo.is_none() {
                if let Some(col) = objeto.material().color_ambiente {
                    colores.push(col * luz.atenuación(punto));
                }
            }
        }
        */

        color
    }

    fn intersecar_rayo(&self, rayo: &Rayo) -> Option<(&'a dyn Modelo, f64)> {
        // el objeto más cercano que atraviesa el rayo
        let menor = self.objetos.iter()
            .map(|obj| (obj, obj.chocan(&rayo)) )
            .filter(|(_, t)| t.is_some())
            .reduce(|menor, actual| {
                let t_menor = menor.1.unwrap();
                let t_actual = actual.1.unwrap();
                if t_actual < t_menor {
                    actual
                } else {
                    menor
                }
            });

        match menor {
            Some((obj, Some(t))) => {
                Some((*obj, t))
            }
            _ => {
                None
            }
        }
    }
}

