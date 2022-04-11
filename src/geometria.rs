use nalgebra::{Matrix3, Point3, Vector3};

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

            Some((t, u, v))
        }
        None  => { None }
    }
}

/// Devuelve un versor aleatorio con función densidad p(t) = (1/pi)*cos(t), con t el ángulo entre
/// el versor generado y la normal pasada como parámetro. o sea es más probable que el versor esté
/// cerca de la normal
pub fn versor_aleatorio_densidad_cos(normal: &Vector3<f64>) -> Vector3<f64> {
    let sen_tita = rand::random::<f64>().sqrt();        // sen(θ) = sqrt(R_1)
    let cos_tita = (1.0 - sen_tita*sen_tita).sqrt();    // cos(θ) = sqrt( 1 - sen(θ)² )
    let phi: f64 = 2.0 * std::f64::consts::PI * rand::random::<f64>();  // φ = 2.π.R_2

    let v: Vector3<f64> = Vector3::new(phi.cos() * sen_tita, phi.sin() * sen_tita, cos_tita);

    crear_base_usando_normal(normal) * v
}

/// Devuelve una matriz de cambio de base a la canónica, siendo la base original una creada tomando
/// el versor k, y dos versores cualquiera que sean ortogonales a k
fn crear_base_usando_normal(normal: &Vector3<f64>) -> Matrix3<f64> {
    let mut b_1: Vector3<f64>; 

    // si la normal está cerca del eje X uso el eje Y, si no uso el X
    if normal.x > 0.9 {
        b_1 = Vector3::new(0.0, 1.0, 0.0);
    } else {
        b_1 = Vector3::new(1.0, 0.0, 0.0);
    }

    b_1 -= normal * b_1.dot(&normal);   // b_1 ortogonal a normal
    b_1 *= b_1.norm();                  // b_1 normalizado

    let b_2 = normal.cross(&b_1);

    Matrix3::from_columns(&[b_1, b_2, *normal])
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

