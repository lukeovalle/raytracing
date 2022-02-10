use image::{ImageBuffer, Rgb, Pixel};
use crate::camara::Cámara;
use crate::modelos::Modelo;

struct Luz {}

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

    pub fn renderizar(&self) -> Result<ImageBuffer<Rgb<u8>, Vec<<Rgb<u8> as Pixel>::Subpixel>>, anyhow::Error> 
    {
        let mut buffer_img = ImageBuffer::new(self.cámara.ancho(), self.cámara.alto());

        for (x, y, pixel) in buffer_img.enumerate_pixels_mut() {
            let rayo = self.cámara.lanzar_rayo(x, y);

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

            if let Some((obj, _)) = menor {
                let color = obj.color_del_rayo(&rayo);

                if let Some(col) = color {
                    *pixel = Rgb([
                        (col.x * 256.0) as u8,
                        (col.y * 256.0) as u8,
                        (col.z * 256.0) as u8
                    ]);
                } else {
                    let normalizado = rayo.dirección().normalize();
                    let color = (normalizado.x + normalizado.y + normalizado.z) / 3.0;
                    *pixel = Rgb([
                        (color * 256.0) as u8,
                        (color * 256.0) as u8,
                        (color * 256.0) as u8
                    ]);

                }
            }
        }

        Ok(buffer_img)
    }

//    fn trazar_rayo(&self, &Rayo) {
//    }
}

