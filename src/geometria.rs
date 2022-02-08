use nalgebra::{Point3, Vector3};

pub type Punto = Point3<f64>;

pub struct Rayo {
    origen: Punto,
    dirección: Vector3<f64>
}

impl Rayo {
    pub fn new(origen: &Punto, dirección: &Vector3<f64>) -> Rayo {
        Rayo {
            origen: origen.clone(),
            dirección: dirección.clone();
        }
    }

    pub fn evaluar(&self, t: f64) -> Punto {
        self.origen + self.dirección * t
    }
}

pub struct Rectángulo(Punto, Punto, Punto, Punto);


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evaluar_rayo() {
        let rayo = Rayo::new(Punto(1.0, 2.0, 3.0), dirección: Vector3<f64>(3.0, 2.0, 1.0));

        assert!((rayo.evaluar(1.0) - Punto(4.0, 4.0, 4.0)).abs() < f64::EPSILON);
    }
}

