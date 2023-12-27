use crate::auxiliar::{bigger_of_three, smaller_of_three};
use crate::geometry::{
    intersect_ray_and_triangle, Normal, Point, Ray, Transform, AABB,
};
use crate::material::Material;
use crate::shapes::{Intersection, Shape, ShapeOperations};

#[derive(Clone, Copy, Debug)]
pub struct Triangle {
    vértices: [Point; 3],
    local_to_world: Transform,
    material: Material,
    caja: AABB, // bounding box en coordenadas globales
    normal: Normal,
}

impl Triangle {
    pub fn new(
        p_1: &Point,
        p_2: &Point,
        p_3: &Point,
        local_to_world: &Transform,
        material: &Material,
    ) -> Triangle {
        Triangle {
            vértices: [*p_1, *p_2, *p_3],
            local_to_world: *local_to_world,
            material: *material,
            caja: Triangle::get_box(p_1, p_2, p_3, local_to_world),
            normal: (p_2 - p_1).cross(&(p_3 - p_1)).normalize(),
        }
    }

    /// Calcula el bounding box de un triángulo, pasando los puntos a coordenadas globales
    fn get_box(
        p_1: &Point,
        p_2: &Point,
        p_3: &Point,
        local_to_world: &Transform,
    ) -> AABB {
        let p_1 = local_to_world * p_1;
        let p_2 = local_to_world * p_2;
        let p_3 = local_to_world * p_3;

        let min = Point::new(
            smaller_of_three(p_1.x, p_2.x, p_3.x),
            smaller_of_three(p_1.y, p_2.y, p_3.y),
            smaller_of_three(p_1.z, p_2.z, p_3.z),
        );
        let max = Point::new(
            bigger_of_three(p_1.x, p_2.x, p_3.x),
            bigger_of_three(p_1.y, p_2.y, p_3.y),
            bigger_of_three(p_1.z, p_2.z, p_3.z),
        );

        AABB::new(&min, &max)
    }

    pub fn vértice(&self, i: usize) -> Point {
        self.vértices[i]
    }

    fn normal(&self, _punto: &Point) -> Normal {
        self.normal
    }
}

impl ShapeOperations for Triangle {
    fn material(&self) -> &Material {
        &self.material
    }

    fn intersect(&self, rayo: &Ray) -> Option<Intersection> {
        let local_ray = self.local_to_world.inverse() * rayo;

        match intersect_ray_and_triangle(&self.vértices, &local_ray) {
            Some((t, ..)) => {
                let punto = match local_ray.at(t) {
                    Some(p) => self.local_to_world * p,
                    None => return None,
                };

                let normal = self.local_to_world * self.normal(&punto);

                let model = Shape::from(*self);

                Some(Intersection::new(
                    &model,
                    &punto,
                    rayo,
                    &normal.normalize(),
                    t / normal.norm(),
                ))
            }
            None => None,
        }
    }

    fn bounding_box(&self) -> &AABB {
        &self.caja
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::{self, Vector};
    use crate::{assert_eq_float, assert_eq_vec};

    #[test]
    fn triangle_intersects_ray() {
        let triangle = Triangle::new(
            &Point::new(0.0, 0.0, 0.0),
            &Point::new(1.0, 0.0, 0.0),
            &Point::new(0.0, 1.0, 0.0),
            &Transform::identity(),
            &Material::default(),
        );

        let ray = Ray::new(
            &Point::new(0.0, 0.0, -1.0),
            &Vector::new(0.0, 0.0, 1.0),
            f64::INFINITY,
        );

        let isect = triangle.intersect(&ray);

        assert!(isect.is_some());
        let isect = isect.unwrap();

        assert_eq_float!(isect.t(), 1.0);
        assert_eq_vec!(isect.point(), &Point::new(0.0, 0.0, 0.0));
        assert_eq_vec!(isect.normal(), &Vector::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn triangle_transformed_intersects_ray() {
        let translation =
            geometry::create_translation(&Vector::new(1.0, 0.0, 0.0));
        let rotation = geometry::create_rotation(&Vector::x_axis(), 90.0);
        let scaling = geometry::create_scaling(&Vector::new(2.0, 2.0, 2.0));

        let transform = translation * rotation * scaling;

        let triangle = Triangle::new(
            &Point::new(0.0, 0.0, 0.0),
            &Point::new(1.0, 0.0, 0.0),
            &Point::new(0.0, 1.0, 0.0),
            &transform,
            &Material::default(),
        );

        let ray = Ray::new(
            &Point::new(1.5, -1.0, 1.0),
            &Vector::new(0.0, 1.0, 0.0),
            f64::INFINITY,
        );

        let isect = triangle.intersect(&ray);
        assert!(isect.is_some());
        let isect = isect.unwrap();

        dbg!(isect.clone());
        assert_eq_float!(isect.t(), 1.0);
        assert_eq_vec!(isect.point(), &Point::new(1.5, 0.0, 1.0));
        assert_eq_vec!(isect.normal(), &Vector::new(0.0, 1.0, 0.0));
    }
}
