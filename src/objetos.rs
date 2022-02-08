use geometria::{Punto, Rayo};
use obj::Obj;

pub trait Objeto {
    fn chocan(&self, &Rayo) -> bool;
}


pub struct Polígono {
    modelo: Obj
}

impl Polígono {
    pub fn new(archivo: &String) -> Result<Objeto, anyhow::Error> {
        Ok(Objeto { modelo: Obj::load(&Path::new(archivo))? })
    }
}

impl Objeto for Polígono {
    pub fn chocan(&self, rayo: &Rayo) -> bool {
        true
    }
}

pub struct Esfera {
    centro: Punto,
    radio: f64
}

impl Esfera {
    pub fn new(centro: &Punto, radio: f64) -> Esfera {
        Esfera {
            centro: Punto.clone(),
            radio
        }
    }
}

impl Objeto for Esfera {
    pub fn chocan(&self, rayo: &Rayo) -> bool {
        // C centro de la esfera, r radio, P+X.t rayo. busco t de intersección
        // (P + t.X - C) * (P + t.X - C) - r² = 0
        // términos cuadráticos: a = X*X, b = 2.X.(P-C), c = (P-C)*(P-C)-r²
        let a = rayo.dirección.dot(rayo.dirección);
        let b = 2.0 * rayo.dirección.dot(rayo.origen - self.centro);
        let c = (rayo.origen - self.centro).dot(rayo.origen - self.centro) - r*r;

        let discriminante = b*b - 4*a*c;
        
        discriminante > 0
    }
}

