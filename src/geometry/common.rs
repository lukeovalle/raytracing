use nalgebra::{Affine3, Point3, Vector3};

pub type Point = Point3<f64>;
pub type Vector = Vector3<f64>;
pub type Normal = Vector3<f64>;

pub type Transform = Affine3<f64>;

pub fn create_point_from_vertex(vertex: &wavefront_obj::obj::Vertex) -> Point {
    Point::new(vertex.x, vertex.y, vertex.z)
}

pub fn create_translation(offset: &Vector) -> Transform {
    Transform::from_matrix_unchecked(
        nalgebra::Translation3::from(*offset).to_homogeneous()
    )
}

pub fn create_rotation(axis: &nalgebra::Unit<Vector>, angle: f64) -> Transform {
    Transform::from_matrix_unchecked(
        nalgebra::Rotation3::from_axis_angle(axis, angle).to_homogeneous()
    )
}

pub fn create_scaling(scale: &Vector) -> Transform {
    Transform::from_matrix_unchecked(
        nalgebra::Scale3::from(*scale).to_homogeneous()
    )
}
