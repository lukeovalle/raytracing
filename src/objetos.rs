use crate::geometria::{Punto, Rayo};
use obj::Obj;

pub trait Objeto {
    fn chocan(&self, rayo: &Rayo) -> bool;
}


pub struct Polígono {
    modelo: Obj
}

impl Polígono {
    pub fn new(archivo: &String) -> Result<Polígono, anyhow::Error> {
        Ok(Polígono { modelo: Obj::load(&std::path::Path::new(archivo))? })
    }
}

impl Objeto for Polígono {
    fn chocan(&self, _rayo: &Rayo) -> bool {
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
            centro: centro.clone(),
            radio
        }
    }
}

impl Objeto for Esfera {
    fn chocan(&self, rayo: &Rayo) -> bool {
        // C centro de la esfera, r radio, P+X.t rayo. busco t de intersección
        // (P + t.X - C) * (P + t.X - C) - r² = 0
        // términos cuadráticos: a = X*X, b = 2.X.(P-C), c = (P-C)*(P-C)-r²
        let a = rayo.dirección().dot(&rayo.dirección());
        let b = 2.0 * rayo.dirección().dot(&(rayo.origen() - self.centro));
        let c = (rayo.origen() - self.centro).dot(&(rayo.origen() - self.centro)) - self.radio*self.radio;

        let discriminante = b*b - 4.0*a*c;
        
        discriminante > 0.0
    }
}

