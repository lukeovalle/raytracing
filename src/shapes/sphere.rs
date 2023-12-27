use super::common::Intersection;
use super::shape::{Shape, ShapeOperations};
use crate::geometry;
use crate::geometry::{Normal, Point, Ray, Transform, Vector, AABB};
use crate::material::Material;

#[derive(Clone, Copy, Debug)]
pub struct Sphere {
    local_to_world: Transform,
    //  world_to_local????
    radio: f64,
    material: Material,
    caja: AABB, // bounding box en coordenadas globales
}

impl Sphere {
    pub fn new(
        transform: &Transform,
        radio: f64,
        material: &Material,
    ) -> Sphere {
        let max = Point::new(radio, radio, radio);
        let min = -max;

        let max = transform * max;
        let min = transform * min;

        Sphere {
            local_to_world: *transform,
            radio,
            material: *material,
            caja: AABB::new(&min, &max),
        }
    }

    /// Devuelve el versor normal en coordenadas globales.
    fn normal(&self, punto: &Point) -> Normal {
        let transform = Transform::from_matrix_unchecked(
            self.local_to_world.inverse().matrix().transpose(),
        );

        (transform * punto.coords).normalize()
    }
}

impl ShapeOperations for Sphere {
    fn material(&self) -> &Material {
        &self.material
    }
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        // paso rayo a coordenadas locales
        let local_ray = self.local_to_world.inverse() * ray;
        let (dir, orig) = (local_ray.dir(), local_ray.origin().coords);

        // r radio, P+X.t rayo, busco t de intersección
        // (P + t.X) * (P + t.X) - r² = 0
        // (P * P - r²) + (2 * P * X) * t + (X * X) * t² = 0
        // términos cuadráticos: a = X.X, b = 2.P.X, c = P.P-r²
        // reemplazando b = 2.h, la ecuación queda (-h+-sqrt(h²-ac))/a
        // simplifico: a = norma²(X), h = P.X, c = norma²(P)-r²
        // X ya viene normalizado de crear el rayo, así que a = 1 siempre

        let h = dir.dot(&orig);
        let c = orig.norm_squared() - self.radio * self.radio;

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

        let punto_local = match local_ray.at(t) {
            Some(p) => p,
            None => return None,
        };

        let model = Shape::from(*self);
        Some(Intersection::new(
            &model,
            &(self.local_to_world * punto_local),
            ray,
            &self.normal(&punto_local),
            t,
        ))
    }

    fn bounding_box(&self) -> &AABB {
        &self.caja
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::{Point, Vector};
    use crate::{assert_eq_float, assert_eq_vec};

    #[test]
    fn sphere_intersects_ray() {
        let sphere =
            Sphere::new(&Transform::identity(), 1.0, &Material::default());

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
    fn translated_sphere_intersects_ray() {
        let sphere = Sphere::new(
            &geometry::create_translation(&Vector::new(2.0, 0.0, 0.0)),
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

    #[test]
    fn aabb_in_sphere_without_transform() {
        let sphere =
            Sphere::new(&Transform::identity(), 1.0, &Material::default());

        let box_ = sphere.bounding_box();

        assert_eq!(box_.min(), Point::new(-1.0, -1.0, -1.0));
        assert_eq!(box_.max(), Point::new(1.0, 1.0, 1.0));
    }

    #[test]
    fn aabb_in_sphere_with_transform() {
        let sphere = Sphere::new(
            &geometry::create_translation(&Vector::new(0.0, 0.0, 1.0)),
            1.0,
            &Material::default(),
        );

        let box_ = sphere.bounding_box();

        assert_eq!(box_.min(), Point::new(-1.0, -1.0, 0.0));
        assert_eq!(box_.max(), Point::new(1.0, 1.0, 2.0));
    }

    #[test]
    fn ray_intersects_uniformly_scaled_sphere() {
        let sphere = Sphere::new(
            &geometry::create_scaling(&Vector::new(0.5, 0.5, 0.5)),
            1.0,
            &Material::default(),
        );

        let ray = Ray::new(
            &Point::new(0.0, 0.0, -1.0),
            &Vector::new(0.0, 0.0, 1.0),
            f64::INFINITY,
        );

        let isect = sphere.intersect(&ray);

        assert!(isect.is_some());
        let isect = isect.unwrap();

        //assert_eq_float!(isect.t(), 0.5);

        dbg!(&isect);
        assert_eq_vec!(isect.point(), &Point::new(0.0, 0.0, -0.5));
        assert_eq_vec!(isect.normal(), &Vector::new(0.0, 0.0, -1.0));
    }

    #[test]
    fn ray_intersects_non_uniformly_scaled_sphere() {
        let scale = &geometry::create_scaling(&Vector::new(1.0, 0.5, 1.0));
        let normal_scale = Transform::from_matrix_unchecked(
            scale.inverse().matrix().transpose(),
        );

        let non_scaled_sphere =
            Sphere::new(&Transform::identity(), 1.0, &Material::default());

        let aux_ray = Ray::new(
            &Point::new(0.0, -2.0, 1.0),
            &Vector::new(0.0, 2.0, -1.0),
            f64::INFINITY,
        );

        let non_scaled_isect = non_scaled_sphere.intersect(&aux_ray).unwrap();
        let expected_point = scale * non_scaled_isect.point();
        let expected_normal =
            (normal_scale * non_scaled_isect.normal()).normalize();

        let sphere = Sphere::new(&scale, 1.0, &Material::default());

        let ray = Ray::new(
            &Point::new(0.0, -1.0, 1.0),
            &Vector::new(0.0, 1.0, -1.0),
            f64::INFINITY,
        );

        let isect = sphere.intersect(&ray);

        assert!(isect.is_some());
        let isect = isect.unwrap();

        dbg!(expected_point);
        dbg!(isect.point());
        dbg!(expected_normal);
        dbg!(isect.normal());

        assert_eq_vec!(isect.point(), &expected_point);
        assert_eq_vec!(isect.normal(), &expected_normal);
    }
}
