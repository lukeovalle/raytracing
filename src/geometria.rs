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


// Möller–Trumbore intersection algorithm
pub fn intersecar_rayo_y_triángulo(vértices: &[Punto], rayo: &Rayo) -> Option<(f64, f64, f64)>{
    // ya fue todo resuelvo con matrices
    let matriz = nalgebra::Matrix3::from_columns(&[
        rayo.dirección() * -1.0,
        vértices[1] - vértices[0],
        vértices[2] - vértices[0]
    ]);

    let b = rayo.origen() - vértices[0];

    let descomposición = matriz.qr();
    let solución = descomposición.solve(&b);

    match solución {
        Some(sol) => {
            let (t, u, v) = (sol[0], sol[1], sol[2]);

            if t < 0.0 || u < 0.0 || v < 0.0 || u + v > 1.0 {
                return None;
            }

            Some((sol[0], sol[1], sol[2]))
        }
        None  => { None }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evaluar_rayo() {
        let rayo = Rayo::new(&Punto::new(1.0, 2.0, 3.0), &Vector3::new(3.0, 2.0, 1.0));

        assert!((rayo.evaluar(1.0) - Punto::new(4.0, 4.0, 4.0)).norm().abs() < f64::EPSILON);
    }

    #[test]
    fn triángulo_interseca_rayo() {
        let vértices = [
            &Punto::new(1.0, -1.0, 0.0),
            &Punto::new(1.0,  1.0, 0.0),
            &Punto::new(1.0,  0.0, 1.0)
        ];
        let rayo = Rayo::new(&Punto::new(0.0, 0.0, 0.0), &Vector3::new(1.0, 0.0, 0.5));

        assert!(intersecar_rayo_y_triángulo(&vértices, &rayo).is_some());
    }

    #[test]
    fn triángulo_no_interseca_rayo() {
        let vértices = [
            &Punto::new(1.0, -1.0, 0.0),
            &Punto::new(1.0,  1.0, 0.0),
            &Punto::new(1.0,  0.0, 1.0)
        ];
        let rayo = Rayo::new(&Punto::new(0.0, 0.0, 0.0), &Vector3::new(1.0, 3.0, 3.0));

        assert!(intersecar_rayo_y_triángulo(&vértices, &rayo).is_none());
    }
}

