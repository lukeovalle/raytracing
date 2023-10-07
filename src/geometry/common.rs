use nalgebra::{Point3, Vector3};

pub type Point = Point3<f64>;
pub type Vector = Vector3<f64>;
pub type Normal = Vector3<f64>;

pub fn create_point_from_vertex(vertex: &wavefront_obj::obj::Vertex) -> Point {
    Point::new(vertex.x, vertex.y, vertex.z)
}


