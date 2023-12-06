mod aabb;
mod common;
mod ray;

pub use aabb::AABB;
pub use common::*;
pub use ray::Ray;

use nalgebra::Matrix3;

#[derive(Clone, Copy, Debug)]
pub struct Rectangle(pub Point, pub Point, pub Point, pub Point);

// Möller–Trumbore intersection algorithm
#[allow(non_snake_case)]
pub fn intersect_ray_and_triangle(
    vértices: &[Point],
    rayo: &Ray,
) -> Option<(f64, f64, f64)> {
    // Buscá el algoritmo ese en wikipedia, hay un pdf.
    // sean:
    // D: dirección normalizada del rayo
    // O: origen del rayo
    // V_0, V_1, V_2: vectores del triángulo
    // E_1 = V_1 - V_0
    // E_2 = V_2 - V_0
    // T = O - V_0
    // Resolver el sistema de ecuaciones [-D, E_1, E_2].[t, u, v]^t = T
    // [ -D.x | E_1.x | E_2.x ]   [ t ]   [ T.x ]
    // [ -D.y | E_1.y | E_2.y ] . [ u ] = [ T.y ]
    // [ -D.z | E_1.z | E_2.z ]   [ v ]   [ T.z ]
    //
    // Aplicando la regla de Cramer,
    // [ t ]          1           [ |  T, E_1, E_2 | ]
    // [ u ] = ---------------- . [ | -D,  T , E_2 | ]  (lo de la derecha es un
    // [ v ]   | -D, E_1, E_2 |   [ | -D, E_1,  T  | ]   vector)
    //
    // propiedad de determinantes: |u, v, w| = -(u x w).v = -(w x v).u
    // Reemplazo con:
    // P = D x E_2
    // Q = T x E_1
    // [ t ]     1     [ Q.E_2 ]
    // [ u ] = ----- . [  P.T  ]
    // [ v ]   P.E_1   [  Q.D  ]

    // Preparo E_1, E_2, T, D
    let E_1 = vértices[1] - vértices[0];
    let E_2 = vértices[2] - vértices[0];
    let T = rayo.origin() - vértices[0];
    let D = rayo.dir();

    // creo P, Q
    let P = D.cross(&E_2);
    let Q = T.cross(&E_1);

    // busco determinante
    let det = P.dot(&E_1);
    if det.abs() < 1e-10 {
        return None;
    }
    let det_inverso = det.recip();

    let t = det_inverso * Q.dot(&E_2);
    if t < 0.0 {
        return None;
    }

    let u = det_inverso * P.dot(&T);
    if !(0.0..=1.0).contains(&u) {
        //    if u < 0.0 || u > 1.0 {
        return None;
    }

    let v = det_inverso * Q.dot(D);
    if v < 0.0 || u + v > 1.0 {
        return None;
    }

    Some((t, u, v))
}

/// Devuelve un versor aleatorio con función densidad p(t) = (1/pi)*cos(t), con
/// t el ángulo entre el versor generado y la normal pasada como parámetro. O
/// sea es más probable que el versor esté cerca de la normal
pub fn random_versor_cos_density(normal: &Vector) -> Vector {
    // sen(θ) = sqrt(R_1) 
    let sen_tita = rand::random::<f64>().sqrt();
    // cos(θ) = sqrt(1 - sen(θ)²)
    let cos_tita = (1.0 - sen_tita * sen_tita).sqrt();
    // φ = 2.π.R_2
    let phi: f64 = 2.0 * std::f64::consts::PI * rand::random::<f64>();

    let v: Vector =
        Vector::new(phi.cos() * sen_tita, phi.sin() * sen_tita, cos_tita);

    create_base_using_normal(normal) * v
}

/// Devuelve una matriz de cambio de base a la canónica, siendo la base original
/// una creada tomando el versor k, y dos versores cualquiera que sean
/// ortogonales a k
fn create_base_using_normal(normal: &Vector) -> Matrix3<f64> {
    // si la normal está cerca del eje X uso el eje Y, si no uso el X
    let mut b_1 = if normal.x.abs() > 0.9 {
        Vector::new(0.0, 1.0, 0.0)
    } else {
        Vector::new(1.0, 0.0, 0.0)
    };

    b_1 -= normal * b_1.dot(normal); // b_1 ortogonal a normal
    b_1 *= 1.0 / b_1.norm(); // b_1 normalizado

    let b_2 = normal.cross(&b_1);

    Matrix3::from_columns(&[b_1, b_2, *normal])
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{assert_eq_float, assert_eq_vec};

    #[test]
    fn triángulo_interseca_rayo() {
        let vértices = [
            Point::new(1.0, -1.0, 0.0),
            Point::new(1.0, 1.0, 0.0),
            Point::new(1.0, 0.0, 1.0),
        ];
        let rayo =
            Ray::new(
                &Point::new(0.0, 0.0, 0.0),
                &Vector::new(1.0, 0.0, 0.5),
                std::f64::INFINITY,
            );

        assert!(intersect_ray_and_triangle(&vértices, &rayo).is_some());
    }

    #[test]
    fn triángulo_no_interseca_rayo() {
        let vértices = [
            Point::new(1.0, -1.0, 0.0),
            Point::new(1.0, 1.0, 0.0),
            Point::new(1.0, 0.0, 1.0),
        ];
        let rayo =
            Ray::new(
                &Point::new(0.0, 0.0, 0.0),
                &Vector::new(1.0, 3.0, 3.0),
                std::f64::INFINITY,
            );

        assert!(intersect_ray_and_triangle(&vértices, &rayo).is_none());
    }
}

