use nalgebra::{Affine3, Point3, Vector3};

pub type Point = Point3<f64>;
pub type Vector = Vector3<f64>;
pub type Normal = Vector3<f64>;

/// Las transformaciones se tienen que aplicar primero el escalado, después la rotación y por último la traslación.
pub type Transform = Affine3<f64>;

pub fn create_point_from_vertex(vertex: &wavefront_obj::obj::Vertex) -> Point {
    Point::new(vertex.x, vertex.y, vertex.z)
}

pub fn create_translation(offset: &Vector) -> Transform {
    nalgebra::convert(nalgebra::Translation3::from(*offset))
}

pub fn create_rotation(axis: &nalgebra::Unit<Vector>, angle: f64) -> Transform {
    nalgebra::convert(nalgebra::Rotation3::from_axis_angle(axis, angle))
}

pub fn create_scaling(scale: &Vector) -> Transform {
    nalgebra::convert(nalgebra::Scale3::from(*scale))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{assert_eq_float, assert_eq_vec};

    #[test]
    fn compose_transforms() {
        let identity = Transform::identity();
        let translation = create_translation(&Vector::new(1.0, 2.0, 3.0));
        let rotation =
            create_rotation(&Vector::z_axis(), std::f64::consts::FRAC_PI_2);
        let scaling = create_scaling(&Vector::new(2.0, 3.0, 4.0));

        let point = Point::new(1.0, 2.0, 3.0);

        assert_eq_vec!(
            rotation * (scaling * point),
            (rotation * scaling) * point
        );
        assert_eq_vec!(
            rotation * (scaling * point),
            rotation * scaling * point
        );
        assert_eq_vec!(
            rotation * (scaling * point),
            &Point::new(-6.0, 2.0, 12.0)
        );
        assert_eq_vec!(
            translation * rotation * scaling * point,
            (translation * rotation * scaling) * point
        );
        assert_eq_vec!(
            translation * rotation * scaling * point,
            &Point::new(-5.0, 4.0, 15.0)
        );
    }

    #[test]
    fn transform_point() {
        let translation = create_translation(&Vector::new(1.0, 2.0, 3.0));
        let rotation =
            create_rotation(&Vector::z_axis(), std::f64::consts::FRAC_PI_2);
        let scaling = create_scaling(&Vector::new(2.0, 3.0, 4.0));

        let point = Point::new(1.0, 2.0, 3.0);

        assert_eq_vec!(scaling * point, &Point::new(2.0, 6.0, 12.0));
        assert_eq_vec!(rotation * point, &Point::new(-2.0, 1.0, 3.0));
        assert_eq_vec!(translation * point, &Point::new(2.0, 4.0, 6.0));
        assert_eq_vec!(
            translation * rotation * scaling * point,
            &Point::new(-5.0, 4.0, 15.0)
        );
    }

    #[test]
    fn transform_vector() {
        let translation = create_translation(&Vector::new(1.0, 2.0, 3.0));
        let rotation =
            create_rotation(&Vector::z_axis(), std::f64::consts::FRAC_PI_2);
        let scaling = create_scaling(&Vector::new(2.0, 3.0, 4.0));

        let vector = Vector::new(1.0, 2.0, 3.0);

        assert_eq_vec!(scaling * vector, &Vector::new(2.0, 6.0, 12.0));
        assert_eq_vec!(rotation * vector, &Vector::new(-2.0, 1.0, 3.0));
        assert_eq_vec!(translation * vector, &Vector::new(1.0, 2.0, 3.0));
        assert_eq_vec!(
            translation * rotation * scaling * vector,
            &Vector::new(-6.0, 2.0, 12.0)
        );
    }
}
