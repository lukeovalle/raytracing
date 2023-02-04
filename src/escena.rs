use image::{ImageBuffer, Rgb, Pixel};
use indicatif::{ProgressBar, ProgressStyle};
use crate::camara::Cámara;
use crate::modelos::{Choque, Modelo};
use crate::material::{Tipo, Color, mezclar_colores, sumar_colores};
use crate::geometria::Rayo;

pub struct Escena<'a> {
    cámara: Cámara,
    objetos: Vec<&'a dyn Modelo>
}

impl<'a> Escena<'a> {
    pub fn new(cámara: &Cámara) -> Escena {
        Escena {
            cámara: *cámara,
            objetos: Vec::new()
        }
    }

    pub fn añadir_objeto(&mut self, objeto: &'a (dyn Modelo + 'a)) -> Result<(), anyhow::Error> {
        self.objetos.push(objeto);
        Ok(())
    }

    pub fn renderizar(&self) -> Result<ImageBuffer<Rgb<u8>, Vec<<Rgb<u8> as Pixel>::Subpixel>>, anyhow::Error> 
    {
        let mut buffer_img = ImageBuffer::new(self.cámara.ancho(), self.cámara.alto());

        // barrita de carga
        let barrita = ProgressBar::new(self.cámara.ancho() as u64 * self.cámara.alto() as u64);

        barrita.set_style(ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise} ({duration} estimado)] [{wide_bar:.cyan/blue}] {percent}%")?
            .progress_chars("#>-")
        );

        for (x, y, pixel) in buffer_img.enumerate_pixels_mut() {
            // Integración de Monte Carlo
            let muestras_por_pixel = 300;
            let colores: Vec<Color> = (0..muestras_por_pixel).map(|_| {
                let (v_1, v_2): (f64, f64) = (rand::random(), rand::random());
                let rayo = self.cámara.lanzar_rayo(x as f64 + v_1 - 0.5 , y as f64 + v_2 - 0.5);
                
                self.trazar_rayo(&rayo, 10)
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
            return Color::zeros();
        }

        match self.intersecar_rayo(rayo) {
            Some(mut choque) => {
                if choque.normal().dot(rayo.dirección()) > 0.0 {
                    // la normal apunta para "adentro" del objeto
                    choque.invertir_normal()
                } else {
                    // apunta para afuera
                    // no hago nada por ahora
                }

                // devuelvo el color en el punto
                self.sombrear_punto(&choque, iteraciones)
            }
            None => { Color::zeros() }
        }
    }

    fn sombrear_punto(
        &self,
        choque: &Choque,
        iteraciones: usize
    ) -> Color {
        let objeto = choque.modelo();
        let punto = choque.punto();
        let incidente = choque.rayo_incidente().dirección();
        let normal = choque.normal();

        match objeto.material().tipo {
            Tipo::Emisor => {
                if let Some(col) = objeto.material().color_emitido {
                    col
                } else {
                    Color::zeros()
                }
            }
            Tipo::Lambertiano => {
                let dirección = crate::geometria::versor_aleatorio_densidad_cos(normal);
                let rayo = Rayo::new(&(punto + normal * 1e-10), &dirección);

                if let Some(col) = objeto.material().color_ambiente {
                    //sumar_colores(&self.trazar_rayo(&rayo, iteraciones - 1), &col)
                    self.trazar_rayo(&rayo, iteraciones - 1).component_mul(&col)
                } else {
                    Color::zeros()
                }
            }
            Tipo::Especular => {
                let color = if let Some(col) = objeto.material().color_especular {
                    col
                } else {
                    Color::new(1.0, 1.0, 1.0)
                };

                // si i es el rayo incidente, n es la normal, y r el reflejado respecto a esa
                // normal, entonces r = i + 2.a, 2.a es la diferencia entre ambos vectores.
                // a tiene dirección de n y módulo i*cos(angulo(i,n)). o sea a = <d, n>.n
                // Asumo que n viene normalizado
                let dirección = incidente - normal * (2.0 * incidente.dot(normal));

                let rayo = Rayo::new(&(punto + normal * 1e-10), &dirección);

                let tita = normal.dot(&dirección);
                // Aproximación de Schlick a las ecuaciones de Fresnel
                // R(t) = R_0 + (1 - R_0)*(1 - cos(t))⁵
                let color = color.map(|r| r + (1.0 - r) * (1.0 - tita.cos()).powi(5));

                //sumar_colores(&self.trazar_rayo(&rayo, iteraciones - 1), &color)
                self.trazar_rayo(&rayo, iteraciones - 1).component_mul(&color)
            }
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
    }

    // Si el rayo choca contra algo, devuelve el coso chocado y el t a evaluar en el rayo para el
    // choque.
    fn intersecar_rayo(&self, rayo: &Rayo) -> Option<Choque> {
        // el objeto más cercano que atraviesa el rayo
        let menor = self.objetos.iter()
            .filter_map(|obj| obj.chocan(rayo) )
            .reduce(|menor, actual| {
                if actual.t() < menor.t() {
                    actual
                } else {
                    menor
                }
            });
        
        menor
    }
}

