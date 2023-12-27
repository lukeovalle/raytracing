use crate::geometry::{Ray, AABB};
use crate::material::Material;
use crate::shapes::{Intersection, Shape, ShapeOperations};

#[allow(clippy::upper_case_acronyms)]
#[derive(Clone, Debug)]
pub struct BoxAABB {
    objetos: Vec<Shape>,
    mat: Material, // No lo uso, está para devolver algo
    caja: AABB,
}

impl BoxAABB {
    pub fn new() -> BoxAABB {
        BoxAABB {
            objetos: Vec::new(),
            mat: Default::default(),
            caja: AABB::empty(),
        }
    }

    pub fn add_model(&mut self, modelo: &Shape) {
        self.caja.resize_box(modelo.bounding_box());
        self.objetos.push(modelo.clone());
    }

    fn intersection_ray_box(&self, rayo: &Ray) -> bool {
        self.caja.intersect_ray(rayo).is_some()
    }
}

impl ShapeOperations for BoxAABB {
    fn material(&self) -> &Material {
        &self.mat
    }

    fn intersect(&self, rayo: &Ray) -> Option<Intersection> {
        if !self.intersection_ray_box(rayo) {
            return None;
        }

        for obj in &self.objetos {
            let choque = obj.intersect(rayo);
            if choque.is_some() {
                return choque;
            }
        }
        None
    }

    fn bounding_box(&self) -> &AABB {
        &self.caja
    }
}
