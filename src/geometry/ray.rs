use std::ops::Mul;
use super::common::*;

#[derive(Clone, Copy, Debug)]
pub struct Ray {
    origin: Point,
    dir: Vector, // debe ser normalizado en coordenadas globales. en coordenadas locales sirve para el escalado
    max_t: f64,
    // también debería guardar el medio de transmisión de la luz (humo, etc.)?
}

impl Ray {
    pub fn new(origin: &Point, dir: &Vector, max_t: f64) -> Ray {
        Ray {
            origin: *origin,
            dir: dir.clone().normalize(),
            max_t,
        }
    }

    pub fn origin(&self) -> &Point {
        &self.origin
    }

    pub fn dir(&self) -> &Vector {
        &self.dir
    }

    #[inline]
    pub fn at(&self, t: f64) -> Option<Point> {
        if t < 0.0 || t > self.max_t {
            return None;
        }

        Some(self.origin + self.dir * t)
    }
}

impl Mul<&Ray> for Transform {
    type Output = Ray;

    fn mul(self, rhs: &Ray) -> Self::Output {
        Ray {
            origin: self * rhs.origin,
            dir: (self * rhs.dir).normalize(),
            max_t: rhs.max_t, // creo que no hace falta tocar esto
        }
    }
}

impl Mul<Ray> for Transform {
    type Output = Ray  ;

    #[inline]
    fn mul(self, rhs: Ray) -> Self::Output {
        self * &rhs
    }
}

// También debería hacer un struct para RayDifferential después

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{assert_eq_float, assert_eq_vec, geometry};

    #[test]
    fn evaluar_rayo() {
        let rayo =
            Ray::new(
                &Point::new(1.0, 1.0, 2.0),
                &Vector::new(2.0, 2.0, 1.0),
                f64::INFINITY,
            );

        let result = rayo.at(3.0);

        assert!(result.is_some());

        let result = result.unwrap();

        assert_eq_vec!(result, Point::new(3.0, 3.0, 3.0));
    }

    #[test]
    fn translation() {
        let rayo = Ray::new(
            &Point::new(1.0, 0.0, 0.0),
            &Vector::new(0.0, 0.0, 1.0),
            f64::INFINITY,
        );

        let translation = create_translation();

        let result = translation * rayo;
        assert_eq_vec!(result.origin, Point::new(2.0, 0.0, 0.0));
        assert_eq_vec!(result.dir, Vector::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn rotation() {
        let rayo = Ray::new(
            &Point::new(1.0, 0.0, 0.0),
            &Vector::new(0.0, 0.0, 1.0),
            f64::INFINITY,
        );

        let rotation = create_rotation();

        let result = rotation * rayo;
        assert_eq_vec!(result.origin, Point::new(0.0, 0.0, -1.0));
        assert_eq_vec!(result.dir, Vector::new(1.0, 0.0, 0.0));
    }

    #[test]
    fn scaling() {
        let rayo = Ray::new(
            &Point::new(1.0, 0.0, 0.0),
            &Vector::new(0.0, 1.0, 1.0),
            f64::INFINITY,
        );

        let scaling = create_scaling();

        let result = scaling * rayo;
        assert_eq_vec!(result.origin, Point::new(2.0, 0.0, 0.0));
        assert_eq_vec!(result.dir, Vector::new(0.0, 0.5, 3.0).normalize());
    }

    // traslada en (1, 0, 0)
    fn create_translation() -> Transform {
        geometry::create_translation(&Vector::new(1.0, 0.0, 0.0))
    }

    // rota 90 grados en el eje y
    fn create_rotation() -> Transform {
        geometry::create_rotation(&Vector::y_axis(), std::f64::consts::FRAC_PI_2)
    }

    // escala en (2, 0.5, 3)
    fn create_scaling() -> Transform {
        geometry::create_scaling(&Vector::new(2.0, 0.5, 3.0))
    }
}
