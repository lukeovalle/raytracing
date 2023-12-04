use enum_dispatch::enum_dispatch;
use crate::material::Material;
use crate::geometry::AABB;
use crate::shapes::box_aabb::BoxAABB;
use crate::shapes::Intersection;
use crate::shapes::model_obj::ModelObj;
use crate::geometry::Ray;
use crate::shapes::Sphere;
use crate::shapes::triangle::Triangle;

#[enum_dispatch]
pub trait ModelMethods {
    fn material(&self) -> &Material;

    /// Devuelve el valor t en el que hay que evaluar el rayo para el choque,
    /// si es que chocan
    fn intersect(&self, rayo: &Ray) -> Option<Intersection>;

    fn bounding_box(&self) -> &AABB;
}

#[allow(clippy::upper_case_acronyms)]
#[enum_dispatch(ModelMethods)]
#[derive(Clone)]
pub enum Model {
    BoxAABB(BoxAABB),
    Sphere(Sphere),
    Triangle(Triangle),
    ModelObj(ModelObj),
}
