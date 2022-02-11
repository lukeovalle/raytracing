use nalgebra::{Point3, Vector3};

pub type Punto = Point3<f64>;

pub type Color = Vector3<f64>;

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

#[derive(Clone, Copy, Debug)]
pub struct Triángulo {
    vértices: [Punto; 3]
}

impl Triángulo {
    pub fn new(p_1: &Punto, p_2: &Punto, p_3: &Punto) -> Triángulo {
        Triángulo { vértices: [p_1.clone(), p_2.clone(), p_3.clone()] }
    }

    // Möller–Trumbore intersection algorithm
    pub fn intersecar_rayo(&self, rayo: &Rayo) -> Option<(f64, f64, f64)>{
        // ya fue todo resuelvo con matrices
        let matriz = nalgebra::Matrix3::from_columns(&[
            rayo.dirección() * -1.0,
            self.vértices[1] - self.vértices[0],
            self.vértices[2] - self.vértices[0]
        ]);

        let b = rayo.origen() - self.vértices[0];

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

/*
        // Vectores de los bordes del triángulo
        let borde_1 = self.vértices[1] - self.vértices[0];
        let borde_2 = self.vértices[2] - self.vértices[0];

        // trucazo, el determinante de la matriz de columnas |c_1 c_2 c_3| se puede calcular como
        // det = c_1 * (c_2 x c_3) o sea el producto punto y el producto cruz
        let p_vec = rayo.dirección().cross(&borde_2);
        let determinante = borde_1.dot(&p_vec);

        if determinante.abs() < 1e-10 {
            return false;
        }

        // vector entre vértice_0 y origen del rayo
        let t_vec = rayo.origen - self.vértices[0];

        // busco u
        let mut u = t_vec.dot(&p_vec);
        if u < 0.0 || u > determinante {
            return false;
        }

        // busco v
        let q_vec = t_vec.cross(&borde_1);

        let mut v = rayo.dirección().dot(&q_vec);
        if v < 0.0 || u + v > determinante {
            return false;
        }

        // busco t
        let mut t = borde_2.dot(&q_vec);

        let inversa_det = 1.0 / determinante;
        t *= inversa_det;
        u *= inversa_det;
        v *= inversa_det;

        true
*/
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
        let triángulo = Triángulo::new(
            &Punto::new(1.0, -1.0, 0.0),
            &Punto::new(1.0,  1.0, 0.0),
            &Punto::new(1.0,  0.0, 1.0)
        );
        let rayo = Rayo::new(&Punto::new(0.0, 0.0, 0.0), &Vector3::new(1.0, 0.0, 0.5));

        assert!(triángulo.intersecar_rayo(&rayo).is_some());
    }

    #[test]
    fn triángulo_no_interseca_rayo() {
        let triángulo = Triángulo::new(
            &Punto::new(1.0, -1.0, 0.0),
            &Punto::new(1.0,  1.0, 0.0),
            &Punto::new(1.0,  0.0, 1.0)
        );
        let rayo = Rayo::new(&Punto::new(0.0, 0.0, 0.0), &Vector3::new(1.0, 3.0, 3.0));

        assert!(triángulo.intersecar_rayo(&rayo).is_none());
    }
}

