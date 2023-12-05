use crate::auxiliar::{bigger_of_three, smaller_of_three};
use crate::geometry::{AABB, intersect_ray_and_triangle, Normal, Point, Ray};
use crate::material::Material;
use crate::shapes::{Intersection, Shape, ShapeOperations};

#[derive(Clone, Copy, Debug)]
pub struct Triangle {
    vértices: [Point; 3],
    material: Material,
    caja: AABB,
    normal: Normal,
}

impl Triangle {
    pub fn new(
        p_1: &Point,
        p_2: &Point,
        p_3: &Point,
        material: &Material,
    ) -> Triangle {
        Triangle {
            vértices: [*p_1, *p_2, *p_3],
            material: *material,
            caja: Triangle::get_box(p_1, p_2, p_3),
            normal: (p_2 - p_1).cross(&(p_3 - p_1)).normalize(),
        }
    }

    fn get_box(p_1: &Point, p_2: &Point, p_3: &Point) -> AABB {
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
    fn intersect(&self, rayo: &Ray) -> Option<Intersection> {
        match intersect_ray_and_triangle(&self.vértices, rayo) {
            Some((t, ..)) => {
                let punto = match rayo.at(t) {
                    Some(p) => p,
                    None => return None,
                };
                let model = Shape::from(*self);
                Some(Intersection::new(
                    &model,
                    &punto,
                    rayo,
                    &self.normal(&punto),
                    t,
                ))
            }
            None => None,
        }
    }

    fn material(&self) -> &Material {
        &self.material
    }

    fn bounding_box(&self) -> &AABB {
        &self.caja
    }
}
