use crate::geometry;
use crate::geometry::{AABB, Normal, Point, Ray, Transform};
use crate::material::Material;
use super::common::Intersection;
use super::shape::{Shape, ShapeOperations};

#[derive(Clone, Copy)]
pub struct Sphere {
    local_to_world: Transform,
//  world_to_local????
    radio: f64,
    material: Material,
    caja: AABB, // bounding box en coordenadas locales
}

impl Sphere {
    pub fn new(centro: &Point, radio: f64, material: &Material) -> Sphere {
        let max = Point::new(radio, radio, radio);
        let min = -max;

        Sphere {
            local_to_world: geometry::create_translation(&centro.coords),
            radio,
            material: *material,
            caja: AABB::new(&min, &max),
        }
    }

    fn normal(&self, punto: &Point) -> Normal {
        let punto = self.local_to_world.inverse() * punto;
        punto.coords.normalize()
    }
}

impl ShapeOperations for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        // paso rayo a coordenadas locales
        let local_ray = self.local_to_world.inverse() * ray;
        let (dir, orig) = (local_ray.dir(), local_ray.origin());

        // r radio, P+X.t rayo, busco t de intersección
        // (P + t.X) * (P + t.X) - r² = 0
        // (P * P - r²) + (2 * P * X) * t + (X * X) * t² = 0
        // términos cuadráticos: a = X.X, b = 2.P.X, c = P.P-r²
        // reemplazando b = 2.h, la ecuación queda (-h+-sqrt(h²-ac))/a
        // simplifico: a = norma²(X), h = P.X, c = norma²(P)-r²
        // X ya viene normalizado de crear el rayo, así que a = 1 siempre

        let h = dir.dot(&orig.coords);
        let c = orig.coords.norm_squared() - self.radio * self.radio;

        let discriminante = h * h - c;

        // No intersection
        if discriminante < 0.0 {
            return None;
        }

        // t_1 is always smaller than t_2
        let t_1 = -h - discriminante.sqrt();
        let t_2 = -h + discriminante.sqrt();

        // both solutions are in the other direction
        if t_2 < 0.0 {
            return None;
        }

        let t = if t_1 < 0.0 {
            // only t_2 >= 0
            t_2
        } else {
            // both are >= 0 and t_1 is smaller
            t_1
        };

        let punto = match local_ray.at(t) {
            Some(p) => self.local_to_world * p,
            None => return None,
        };

        let model = Shape::from(*self);
        Some(Intersection::new(
            &model,
            &punto,
            ray,
            &self.normal(&punto),
            t,
        ))
    }

    fn material(&self) -> &Material {
        &self.material
    }

    fn bounding_box(&self) -> &AABB {
        &self.caja
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{assert_eq_float, assert_eq_vec};
    use crate::geometry::{Point, Vector};

    #[test]
    fn sphere_interseca_rayo() {
        let sphere = Sphere::new(
            &Point::new(0.0, 0.0, 0.0),
            1.0,
            &Material::default(),
        );

        let ray = Ray::new(
            &Point::new(0.0, 0.0, -5.0),
            &Vector::new(0.0, 0.0, 1.0),
            f64::INFINITY,
        );

        let isect = sphere.intersect(&ray);

        assert!(isect.is_some());

        let isect = isect.unwrap();

        assert_eq_float!(isect.t(), 4.0);
        assert_eq_vec!(isect.point(), &Point::new(0.0, 0.0, -1.0));
        assert_eq_vec!(isect.normal(), &Vector::new(0.0, 0.0, -1.0));
    }

    #[test]
    fn sphere_trasladada_interseca_rayo() {
        let sphere = Sphere::new(
            &Point::new(2.0, 0.0, 0.0),
            1.0,
            &Material::default(),
        );

        let ray = Ray::new(
            &Point::new(0.0, 0.0, 0.0),
            &Vector::new(1.0, 0.0, 0.0),
            f64::INFINITY,
        );

        let isect = sphere.intersect(&ray);

        assert!(isect.is_some());
        let isect = isect.unwrap();

        assert_eq_float!(isect.t(), 1.0);
        assert_eq_vec!(isect.point(), &Point::new(1.0, 0.0, 0.0));
        assert_eq_vec!(isect.normal(), &Vector::new(-1.0, 0.0, 0.0));
    }
}


