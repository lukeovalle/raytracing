use nalgebra::{Point3, Vector3};

pub type Punto = Point3<f64>;

#[derive(Debug)]
pub struct Rayo {
    origen: Punto,
    dirección: Vector3<f64>
}

impl Rayo {
    pub fn new(origen: &Punto, dirección: &Vector3<f64>) -> Rayo {
        Rayo {
            origen: origen.clone(),
            dirección: dirección.clone()
        }
    }

    pub fn origen(&self) -> &Punto {
        &self.origen
    }

    pub fn dirección(&self) -> &Vector3<f64> {
        &self.dirección
    }

    pub fn evaluar(&self, t: f64) -> Punto {
        self.origen + self.dirección * t
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Rectángulo(pub Punto, pub Punto, pub Punto, pub Punto);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evaluar_rayo() {
        let rayo = Rayo::new(&Punto::new(1.0, 2.0, 3.0), &Vector3::new(3.0, 2.0, 1.0));

        assert!((rayo.evaluar(1.0) - Punto::new(4.0, 4.0, 4.0)).norm().abs() < f64::EPSILON);
    }
}

