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
            // Si  tiro varios rayos por el mismo pixel pero corridos un cacho y promedio el
            // resultado, tengo anti-aliasing
            let antialiasing = 5;
            let colores: Vec<Color> = (0..antialiasing).map(|_| {
                let (v_1, v_2): (f64, f64) = (rand::random(), rand::random());
                let rayo = self.cámara.lanzar_rayo(x as f64 + v_1 - 0.5 , y as f64 + v_2 - 0.5);
                
                self.trazar_rayo(&rayo)
            }).collect();

            let color = mezclar_colores(&colores);

            *pixel = Rgb([
                (color.x * 256.0) as u8,
                (color.y * 256.0) as u8,
                (color.z * 256.0) as u8
            ]);

            barrita.set_position(y as u64 * self.cámara.ancho() as u64 + (x + 1) as u64);
        }

        barrita.finish_with_message("ARchivo guardado en archivo.bmp");

        Ok(buffer_img)
    }

    fn trazar_rayo(&self, rayo: &Rayo) -> Color {
        match self.intersecar_rayo(rayo) {
            Some((objeto, t)) => {
                let punto = rayo.evaluar(t);
                let normal = objeto.normal(&punto);

                let color = self.sombrear_punto(objeto, &punto, &normal);

                color
            }
            None => { Color::new(0.6, 0.6, 0.6) }
        }
    }

    fn sombrear_punto(
        &self,
        objeto: &'a (dyn Modelo + 'a),
        punto: &Punto,
        normal: &Vector3<f64>
    ) -> Color {
        let mut colores = Vec::new();

        if let Some(col) = objeto.material().color_emitido {
            colores.push(col);
        }

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

        if colores.is_empty() {
            Color::new(0.0, 0.0, 0.0)
        } else {
            mezclar_colores(&colores)
        }
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

