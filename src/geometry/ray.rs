use super::common::*;

#[derive(Clone, Copy, Debug)]
pub struct Ray {
    origin: Point,
    dir: Vector,
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

// También debería hacer un struct para RayDifferential después

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{assert_eq_float, assert_eq_vec};

    #[test]
    fn evaluar_rayo() {
        let rayo =
            Ray::new(
                &Point::new(1.0, 1.0, 2.0),
                &Vector::new(2.0, 2.0, 1.0),
                std::f64::INFINITY,
            );

        let result = rayo.at(3.0);

        assert!(result.is_some());
        assert_eq_vec!(rayo.at(3.0).unwrap(), Point::new(3.0, 3.0, 3.0));
    }


}
