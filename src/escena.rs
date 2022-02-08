use image::ImageBuffer;
use camara::Cámara;
use objeto::Objeto;

pub struct Escena {
    cámara: Cámara,
    objetos: Vec<Objeto>,
    luces: Vec<Luz>
}

impl Escena {
    pub fn new(cámara: &Cámara) -> Escena {
        Escena {
            cámara: cámara.clone(),
            objetos: Vec::new(),
            luces: Vec::new()
        }
    }

    pub fn añadir_objeto(objeto: &Objeto) -> Result<(), anyhow::Error> {
        objetos.push(objeto);
        Ok(())
    }

    pub fn renderizar(&self) -> Result<ImageBuffer, anyhow::Error> {
        let mut buffer_img = ImageBuffer::new(self.cámara.ancho, self.cámara.alto);

        for (x, y, pixel) in buffer_img.enumerate_pixels_mut() {
            rayo = cámara.lanzar_rayo(x, y);

            for objeto in objetos {
                if objeto.chocan(rayo) {
                    *pixel = Rgb([255, 0, 0]);
                } else {
                    let val = (x + y) as f64 / (ancho + alto) as f64 / 256f64;
                    *pixel = Rgb([val as u8, val as u8, val as u8]);
                }
            }
        }
        
        Ok(buffer_img)
    }
}

