use image::{ImageBuffer, Rgb, Pixel};
use crate::camara::Cámara;
use crate::objetos::Objeto;

struct Luz {}

pub struct Escena<'a> {
    cámara: Cámara,
    objetos: Vec<&'a dyn Objeto>,
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

    pub fn añadir_objeto(&mut self, objeto: &'a (dyn Objeto + 'a)) -> Result<(), anyhow::Error> {
        self.objetos.push(objeto);
        Ok(())
    }

//    pub fn renderizar(&self) -> Result<ImageBuffer<dyn Pixel<Subpixel = u8>, Vec<<dyn Pixel<Subpixel = u8> as Pixel>::Subpixel>>, anyhow::Error>
    pub fn renderizar(&self) -> Result<ImageBuffer<Rgb<u8>, Vec<<Rgb<u8> as Pixel>::Subpixel>>, anyhow::Error> 
    {
        let mut buffer_img = ImageBuffer::new(self.cámara.ancho(), self.cámara.alto());

        for (x, y, pixel) in buffer_img.enumerate_pixels_mut() {
            let rayo = self.cámara.lanzar_rayo(x, y);

            for objeto in &self.objetos {
                if objeto.chocan(&rayo) {
                    *pixel = Rgb([255, 0, 0]);
                } else {
                    let val = (x + y) as f64 / (self.cámara.ancho() + self.cámara.alto()) as f64 * 256f64;
                    *pixel = Rgb([val as u8, val as u8, val as u8]);
                }
            }
        }
        
        Ok(buffer_img)
    }
}

