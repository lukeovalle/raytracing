use super::common::*;
use super::ray::Ray;

/// Hexaedro con caras ortogonales al sistema de coordenadas globales, de cuatro
/// lados y ángulos rectos.
/// min y max contiene los componentes (x, y, z) mínimos y máximos de cada
/// vértice.
#[derive(Clone, Copy, Debug)]
pub struct AABB {
    min: Point,
    max: Point,
}

impl AABB {
    pub fn empty() -> AABB {
        AABB {
            min: Point::origin(),
            max: Point::origin(),
        }
    }

    pub fn from_point(p: &Point) -> AABB {
        AABB {
            min: *p,
            max: *p,
        }
    }

    pub fn new(p_1: &Point, p_2: &Point) -> AABB {
        AABB {
            min: Point::new(
                p_1.x.min(p_2.x),
                p_1.y.min(p_2.y),
                p_1.z.min(p_2.z),
            ),
            max: Point::new(
                p_1.x.max(p_2.x),
                p_1.y.max(p_2.y),
                p_1.z.max(p_2.z),
            ),
        }
    }

    pub fn min(&self) -> Point {
        self.min
    }

    pub fn max(&self) -> Point {
        self.max
    }

    pub fn union(&self, other: &AABB) -> AABB {
        let x_min = self.min.x.min(other.min.x);
        let y_min = self.min.y.min(other.min.y);
        let z_min = self.min.z.min(other.min.z);
        let x_max = self.max.x.max(other.max.x);
        let y_max = self.max.y.max(other.max.y);
        let z_max = self.max.z.max(other.max.z);

        AABB {
            min: Point::new(x_min, y_min, z_min),
            max: Point::new(x_max, y_max, z_max),
        }
    }

    pub fn union_point(box_: &AABB, p: &Point) -> AABB {
        let x_min = box_.min.x.min(p.x);
        let y_min = box_.min.y.min(p.y);
        let z_min = box_.min.z.min(p.z);
        let x_max = box_.max.x.max(p.x);
        let y_max = box_.max.y.max(p.y);
        let z_max = box_.max.z.max(p.z);

        AABB {
            min: Point::new(x_min, y_min, z_min),
            max: Point::new(x_max, y_max, z_max),
        }
    }

    pub fn resize_box(&mut self, otra: &AABB) {
        *self = AABB::union(self, otra);
    }

    pub fn intersect(&self, other: &AABB) -> AABB {
        let x_min = self.min.x.max(other.min.x);
        let y_min = self.min.y.max(other.min.y);
        let z_min = self.min.z.max(other.min.z);
        let x_max = self.max.x.min(other.max.x);
        let y_max = self.max.y.min(other.max.y);
        let z_max = self.max.z.min(other.max.z);

        AABB {
            min: Point::new(x_min, y_min, z_min),
            max: Point::new(x_max, y_max, z_max),
        }
    }

    pub fn overlaps(&self, other: &AABB) -> bool {
        let x = self.max.x >= other.min.x && self.min.x <= other.max.x;
        let y = self.max.y >= other.min.y && self.min.y <= other.max.y;
        let z = self.max.z >= other.min.z && self.min.z <= other.max.z;

        x && y && z
    }

    pub fn point_inside(&self, p: &Point) -> bool {
        p.x >= self.min.x && p.x <= self.max.x &&
            p.y >= self.min.y && p.y <= self.max.y &&
            p.z >= self.min.z && p.z <= self.max.z
    }

    pub fn expand(&mut self, delta: f64) {
        let v = Vector::new(delta, delta, delta);

        self.min -= v;
        self.max += v;
    }

    #[inline]
    pub fn diagonal(&self) -> Vector {
        self.max - self.min
    }

    pub fn surface_area(&self) -> f64 {
        let d = self.diagonal();

        2.0 * (d.x * d.y + d.x * d.z + d.y * d.z)
    }

    pub fn volume(&self) -> f64 {
        let d = self.diagonal();

        d.x * d.y * d.z
    }

    pub fn intersect_ray(&self, rayo: &Ray) -> Option<f64> {
        let mut mínimo_intervalo = 0.0;
        let mut máximo_intervalo = f64::INFINITY;

        // Busco t_min y t_max respecto a X
        // La idea es que si alguno de estos es menor a 0, o si t_min es mayor
        // a t_max, entonces la caja no es atravesada por el rayo. La lógica va
        // a ser la misma para los tres ejes, si no puedo probar que no se
        // chocan en cada uno de los tres ejes, entonces se chocan.
        //
        // Se tienen que cumplir seis ecuaciones de 13 incógnitas, estas dos son
        // para x pero la idea es similar en los otros dos ejes.
        // P + t.d = (x_min, 0, 0) + n.(0,1,0) + m.(0,0,1)
        // P + t.d = (x_max, 0, 0) + a.(0,1,0) + b.(0,0,1)
        // con P y d origen y dirección del rayo, x_min y x_max bordes de la
        // caja, y (t,n,m,a,b) incógnitas.
        // de acá se puede despejar con P=(p_x, p_y), d=(d_x, d_y)
        // p_x + t_0.d_x = x_min
        // p_x + t_1.d_x = x_max
        // los casos borde donde el rayo va a chocar con el borde x de la caja
        // sin estar dentro.
        // Considerar que si d_x = 0, entonces solo hay que analizar si
        // x_min < p_x < x_max.
        // De otro modo, se despeja que
        // t_0 = (x_min - p_x)/d_x y t_1 = (x_max - p_x)/d_x
        // Considerando que x_min y x_max tienen un orden conocido, para saber
        // en que caso t_min = t_0 y en que caso t_min = t_1 (y lo mismo para
        // t_max), hay que analizar la dirección d_x. Si d_x > 0, t_min = t_0,
        // si d_x < 0, t_min = t_1
        //
        // Toda esta lógica se aplica de igual manera para los ejes Y y Z

        if rayo.dir().x == 0.0 {
            if rayo.origin().x < self.min.x || rayo.origin().x > self.max.x {
                return None;
            }
        } else {
            let t_0 = (self.min.x - rayo.origin().x) / rayo.dir().x;
            let t_1 = (self.max.x - rayo.origin().x) / rayo.dir().x;

            let (t_min, t_max) = if rayo.dir().x > 0.0 {
                (t_0, t_1)
            } else {
                (t_1, t_0)
            };

            if t_min > mínimo_intervalo {
                mínimo_intervalo = t_min;
            }
            if t_max < máximo_intervalo {
                máximo_intervalo = t_max;
            }

            if mínimo_intervalo > máximo_intervalo {
                return None;
            }
        }

        // En el eje Y
        if rayo.dir().y == 0.0 {
            if rayo.origin().y < self.min.y || rayo.origin().y > self.max.y {
                return None;
            }
        } else {
            let t_0 = (self.min.y - rayo.origin().y) / rayo.dir().y;
            let t_1 = (self.max.y - rayo.origin().y) / rayo.dir().y;

            let (t_min, t_max) = if rayo.dir().y > 0.0 {
                (t_0, t_1)
            } else {
                (t_1, t_0)
            };

            if t_min > mínimo_intervalo {
                mínimo_intervalo = t_min;
            }
            if t_max < máximo_intervalo {
                máximo_intervalo = t_max;
            }
            if mínimo_intervalo > máximo_intervalo {
                return None;
            }
        }

        // En el eje Z
        if rayo.dir().z == 0.0 {
            if rayo.origin().z < self.min.z || rayo.origin().z > self.max.z {
                return None;
            }
        } else {
            let t_0 = (self.min.z - rayo.origin().z) / rayo.dir().z;
            let t_1 = (self.max.z - rayo.origin().z) / rayo.dir().z;

            let (t_min, t_max) = if rayo.dir().z > 0.0 {
                (t_0, t_1)
            } else {
                (t_1, t_0)
            };

            if t_min > mínimo_intervalo {
                mínimo_intervalo = t_min;
            }
            if t_max < máximo_intervalo {
                máximo_intervalo = t_max;
            }
            if mínimo_intervalo > máximo_intervalo {
                return None;
            }
        }

        Some(mínimo_intervalo)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{assert_eq_float, assert_eq_vec};

    #[test]
    fn caja_interseca_rayo() {
        let caja = AABB::new(
            &Point::new(1.0, 1.0, 1.0),
            &Point::new(2.0, 2.0, 2.0),
        );
        let rayo =
            Ray::new(
                &Point::new(0.0, 0.0, 0.0),
                &Vector::new(1.5, 1.5, 1.5),
                f64::INFINITY,
            );

        assert!(caja.intersect_ray(&rayo).is_some());
    }

    #[test]
    fn caja_no_interseca_rayo() {
        let caja = AABB::new(
            &Point::new(1.0, 1.0, 1.0),
            &Point::new(2.0, 2.0, 2.0),
        );
        let rayo =
            Ray::new(
                &Point::new(0.0, 0.0, 0.0),
                &Vector::new(1.0, 0.0, 0.0),
                f64::INFINITY,
            );

        assert!(caja.intersect_ray(&rayo).is_none());
    }

    #[test]
    fn rayo_adentro_de_caja() {
        let caja = AABB::new(
            &Point::new(0.0, 0.0, 0.0),
            &Point::new(2.0, 2.0, 2.0),
        );
        let rayo =
            Ray::new(
                &Point::new(1.0, 1.0, 1.0),
                &Vector::new(1.0, 0.0, 0.0),
                f64::INFINITY,
            );

        assert!(caja.intersect_ray(&rayo).is_some());
    }

    #[test]
    fn unir_cajas() {
        let c_1 = AABB::new(
            &Point::new(1.0, 1.0, 1.0),
            &Point::new(2.0, 2.0, 2.0),
        );
        let c_2 = AABB::new(
            &Point::new(0.0, 0.0, 0.0),
            &Point::new(1.0, 1.0, 1.0),
        );

        let caja = c_1.union(&c_2);

        assert_eq_vec!(caja.min, Point::new(0.0, 0.0, 0.0));
        assert_eq_vec!(caja.max, Point::new(2.0, 2.0, 2.0));
    }

    #[test]
    fn ampliar_caja() {
        let mut caja = AABB::new(
            &Point::new(1.0, 1.0, 1.0),
            &Point::new(2.0, 2.0, 2.0),
        );
        let c_2 = AABB::new(
            &Point::new(0.0, 0.0, 0.0),
            &Point::new(1.0, 1.0, 1.0),
        );

        caja.resize_box(&c_2);

        assert_eq_vec!(caja.min, Point::new(0.0, 0.0, 0.0));
        assert_eq_vec!(caja.max, Point::new(2.0, 2.0, 2.0));
    }
}
