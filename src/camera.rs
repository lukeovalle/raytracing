use crate::geometry::{Point, Ray, Rectangle};

#[derive(Clone, Copy, Debug)]
pub struct Camera {
    focus: Point,
    screen: Rectangle,
    width: u32,
    height: u32,
}

impl Camera {
    /// focus: el Point del centro de la cámara
    /// focal_distance: distancia entre el foco y la pantalla
    /// field_of_view: ángulo en grados entre los lados de la pantalla, con centro en el foco
    /// rotation: rotación en radianes de los ejes X, Y, Z (roll, pitch, yaw).
    ///              (0,0,0) es el plano YZ en dirección +X
    /// resolution: ancho y alto de la imagen en pixeles
    pub fn new(
        focus: &Point,
        focal_distance: f64,
        field_of_view: f64,
        rotation: (f64, f64, f64),
        resolution: (u32, u32),
    ) -> Camera {
        // p_1 es la esquina de arriba a la izquierda
        // p_2 es la esquina de arriba a la derecha
        // p_3 es la esquina de abajo a la izquierda
        // p_4 es la esquina de abajo a la derecha

        let delta_y = focal_distance
            * (field_of_view / 2.0 * std::f64::consts::PI / 180.0).tan();
        let delta_z = delta_y * resolution.1 as f64 / resolution.0 as f64;

        let rot = nalgebra::Rotation3::from_euler_angles(
            rotation.0, rotation.1, rotation.2,
        );
        let tras = nalgebra::Translation3::new(focus.x, focus.y, focus.z);

        let p_1 = tras * rot * Point::new(focal_distance, -delta_y, delta_z);
        let p_2 = tras * rot * Point::new(focal_distance, delta_y, delta_z);
        let p_3 = tras * rot * Point::new(focal_distance, -delta_y, -delta_z);
        let p_4 = tras * rot * Point::new(focal_distance, delta_y, -delta_z);

        Camera {
            focus: *focus,
            screen: Rectangle(p_1, p_2, p_3, p_4),
            width: resolution.0,
            height: resolution.1,
        }
    }

    /*
    pub fn focus(&self) -> Point {
        self.focus
    }

    pub fn pantalla(&self) -> &Rectángulo {
        &self.pantalla
    }
    */

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn get_ray(&self, i: f64, j: f64) -> Ray {
        // Me guié con un paint para sacar esta lógica
        // vec_derecha es el vector entre la esquina derecha y la esquina izquierda,
        // escalado por el i en relación con el ancho (el vector es máximo en el último
        // pixel y es nulo en el primer pixel)
        // vec_abajo lo mismo pero de abajo para arriba
        // con eso creo un punto que va de la esquina superior izquierda, hacia abajo a la
        // derecha.
        // el rayo parte del foco y va en dirección punto-foco
        let vec_right = (self.screen.1 - self.screen.0) * i / self.width as f64;
        let vec_down = (self.screen.2 - self.screen.0) * j / self.height as f64;
        let point = self.screen.0 + vec_right + vec_down;

        Ray::new(&self.focus, &(point - self.focus))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn crear_cámara_sin_rotación() {
        let foco = Point::new(0.0, 0.0, 0.0);
        let distancia_focal = 1.0;
        let campo_de_visión = 90.0; // la tangente de 45 es 1
        let resolución = (500, 1000); // relación de aspecto 1:2

        let cámara = Camera::new(
            &foco,
            distancia_focal,
            campo_de_visión,
            (0.0, 0.0, 0.0),
            resolución,
        );

        assert!((cámara.focus.x - foco.x).abs() < 1e-10);
        assert!((cámara.focus.y - foco.y).abs() < 1e-10);
        assert!((cámara.focus.z - foco.z).abs() < 1e-10);
        assert!((cámara.screen.0.x - distancia_focal).abs() < 1e-10);
        assert!((cámara.screen.3.x - distancia_focal).abs() < 1e-10);
        assert!((cámara.screen.0.y + 1.0).abs() < 1e-10);
        assert!((cámara.screen.3.y - 1.0).abs() < 1e-10);
        assert!((cámara.screen.0.z - 2.0).abs() < 1e-10);
        assert!((cámara.screen.3.z + 2.0).abs() < 1e-10);
        assert_eq!(cámara.width, resolución.0);
        assert_eq!(cámara.height, resolución.1);
    }

    #[test]
    fn crear_cámara_con_rotación() {
        let foco = Point::new(0.0, 0.0, 0.0);
        let distancia_focal = 1.0;
        let campo_de_visión = 90.0;
        let resolución = (500, 1000);

        let cámara = Camera::new(
            &foco,
            distancia_focal,
            campo_de_visión,
            (std::f64::consts::PI / 2.0, 0.0, 0.0),
            resolución,
        );

        dbg!(cámara);

        assert!((cámara.focus.x - foco.x).abs() < 1e-10);
        assert!((cámara.focus.y - foco.y).abs() < 1e-10);
        assert!((cámara.focus.z - foco.z).abs() < 1e-10);
        assert!((cámara.screen.0.x - distancia_focal).abs() < 1e-10);
        assert!((cámara.screen.3.x - distancia_focal).abs() < 1e-10);
        assert!((cámara.screen.0.y + 2.0).abs() < 1e-10);
        assert!((cámara.screen.3.y - 2.0).abs() < 1e-10);
        assert!((cámara.screen.0.z + 1.0).abs() < 1e-10);
        assert!((cámara.screen.3.z - 1.0).abs() < 1e-10);
        assert_eq!(cámara.width, resolución.0);
        assert_eq!(cámara.height, resolución.1);
    }

    #[test]
    fn lanzar_rayo() {
        let cámara = Camera::new(
            &Point::new(0.0, 0.0, 0.0), // foco
            1.0,                        // distancia_focal
            90.0,                       // campo_de_visión
            (0.0, 0.0, 0.0),            // rotación
            (100, 100),                 // ancho, alto
        );

        let rayo = cámara.get_ray(0.0, 0.0);
        let dirección_esperada =
            nalgebra::Vector3::new(1.0, -1.0, 1.0).normalize();

        assert!(rayo.origin().x.abs() < 1e-10);
        assert!(rayo.origin().y.abs() < 1e-10);
        assert!(rayo.origin().z.abs() < 1e-10);
        assert!((rayo.direction().x - dirección_esperada.x).abs() < 1e-10);
        assert!((rayo.direction().y - dirección_esperada.y).abs() < 1e-10);
        assert!((rayo.direction().z - dirección_esperada.z).abs() < 1e-10);

        // Pruebo en otro pixel
        let rayo = cámara.get_ray(50.0, 50.0);
        dbg!(&rayo);

        assert!((rayo.direction().x - 1.0).abs() < 1e-10);
        assert!(rayo.direction().y.abs() < 1e-10);
        assert!(rayo.direction().z.abs() < 1e-10);
    }
}
