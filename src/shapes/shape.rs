use super::box_aabb::BoxAABB;
use super::model_obj::ModelObj;
use super::Intersection;
use crate::geometry::Ray;
use crate::geometry::AABB;
use crate::material::Material;
use crate::shapes::triangle::Triangle;
use crate::shapes::Sphere;
use enum_dispatch::enum_dispatch;

#[enum_dispatch]
pub trait ShapeOperations {
    fn material(&self) -> &Material;

    /// Devuelve el valor t en el que hay que evaluar el rayo para el choque,
    /// si es que chocan
    fn intersect(&self, rayo: &Ray) -> Option<Intersection>;

    /// Devuelve true si hay choque, reescribir este método en las implementaciones para que sea más
    /// eficiente.
    fn is_intersecting(&self, ray: &Ray) -> bool {
        self.intersect(ray).is_some()
    }

    fn bounding_box(&self) -> &AABB;

    // Calcula el área de la figura
    //fn area(&self) -> f64;
}

#[allow(clippy::upper_case_acronyms)]
#[enum_dispatch(ShapeOperations)]
#[derive(Clone, Debug)]
pub enum Shape {
    BoxAABB,
    Sphere,
    Triangle,
    ModelObj,
}
