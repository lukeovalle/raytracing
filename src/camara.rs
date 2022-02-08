use geometria::{Punto, Rayo, Rectángulo};

pub struct Cámara {
    foco: Punto,
    pantalla: Rectángulo,
    ancho: u32,
    alto: u32
}

impl Cámara {
    /// foco: el Punto del centro de la cámara
    /// distancia focal: distancia entre el foco y la pantalla
    /// campo_de_visión: ángulo en grados entre los lados de la pantalla, con centro en el foco
    /// inclinación: rotación en radianes de los ejes X, Y, Z (roll, pitch, yaw).
    ///              (0,0,0) es el plano YZ en dirección +X
    /// resolución: ancho y alto de la imagen en pixeles
    pub fn new(
        foco: Punto,
        distancia_focal: f64,
        campo_de_visión: f64
        inclinación: (f64, f64, f64),
        resolución: (u32, u32)
    ) -> Cámara {
        // p_1 es la esquina de arriba a la izquierda
        // p_2 es la esquina de arriba a la derecha
        // p_3 es la esquina de abajo a la izquierda
        // p_4 es la esquina de abajo a la derecha

        let delta_y = distancia_focal * (campo_de_visión / 2 * f64::consts::PI / 180 ).tan();
        let delta_z = delta_y * resolución.1 / resolución.0;

        let rot = nalgebra::Rotation3::from_euler_angles(
            inclinación.0,
            inclinación.1,
            inclinación.2
        );
        let tras = nalgebra::Translation3::new(foco.x, foco.y, foco.z);

        let rototras = nalgebra::Isometry3::from_parts(tras, rot);

        let p_1 = rototras * Punto::new(distancia_focal, -delta_y,  delta_z);
        let p_2 = rototras * Punto::new(distancia_focal,  delta_y,  delta_z);
        let p_3 = rototras * Punto::new(distancia_focal, -delta_y, -delta_z);
        let p_4 = rototras * Punto::new(distancia_focal,  delta_y, -delta_z);


        Cámara {
            foco,
            pantalla: Rectángulo(p_1, p_2, p_3, p_4),
            ancho: resolución.0,
            alto: resolución.1
        }
    }

    pub fn lanzar_rayo(&self, i: u32, j: u32) -> Rayo {
        // Me guié con un paint para sacar esta lógica
        // vec_derecha es el vector entre la esquina derecha y la esquina izquierda,
        // escalado por el i en relación con el ancho (el vector es máximo en el último
        // pixel y es nulo en el primer pixel)
        // vec_abajo lo mismo pero de abajo para arriba
        // con eso creo un punto que va de la esquina superior izquierda, hacia abajo a la
        // derecha.
        // el rayo parte del foco y va en dirección punto-foco
        let vec_derecha = (self.pantalla.1 - self.pantalla.0) * (i / self.ancho);
        let vec_abajo = (self.pantalla.2 - self.pantalla.0) * (j / self.alto);
        let punto = self.pantalla.0 + vec_derecha + vec_abajo;

        Rayo::new(
            self.foco,
            punto - self.foco
        )

    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crear_cámara_sin_rotación() {
        let foco = Punto::new(0.0, 0.0, 0.0);
        let distancia_focal = 1.0;
        let campo_de_visión = 90; // la tangente de 45 es 1
        let resolución = (500, 1000); // relación de aspecto 1:2

        let cámara = Cámara::new(
            foco,
            distancia_focal,
            campo_de_visión,
            (0.0, 0.0, 0.0),
            resolución
        );

        assert!((cámara.foco.x - foco.x).abs() < f64::EPSILON);
        assert!((cámara.foco.y - foco.y).abs() < f64::EPSILON);
        assert!((cámara.foco.z - foco.z).abs() < f64::EPSILON);
        assert!((cámara.pantalla.0.x - distancia_focal).abs() < f64::EPSILON);
        assert!((cámara.pantalla.3.x - distancia_focal).abs() < f64::EPSILON);
        assert!((cámara.pantalla.0.y + 1.0).abs() < f64::EPSILON);
        assert!((cámara.pantalla.3.y - 1.0).abs() < f64::EPSILON);
        assert!((cámara.pantalla.0.z - 2.0).abs() < f64::EPSILON);
        assert!((cámara.pantalla.3.z + 2.0).abs() < f64::EPSILON);
        assert_eq!(cámara.ancho, resolución.0);
        assert_eq!(cámara.alto, resolución.1);
    }

    #[test]
    #[ignore]
    fn test_crear_cámara_con_rotación() {
        let foco = Punto::new(0.0, 0.0, 0.0);
        let distancia_focal = 1.0;
        let campo_de_visión = 120;
        let resolución = (500, 1000);

        let cámara = Cámara::new(
            foco,
            distancia_focal,
            campo_de_visión,
            (1.0, 1.0, 1.0),
            resolución
        );

        assert!((cámara.foco.x - foco.x).abs() < f64::EPSILON);
        assert!((cámara.foco.y - foco.y).abs() < f64::EPSILON);
        assert!((cámara.foco.z - foco.z).abs() < f64::EPSILON);
        assert!((cámara.pantalla.0.x - distancia_focal).abs() < f64::EPSILON);
        assert!((cámara.pantalla.3.x - distancia_focal).abs() < f64::EPSILON);
        assert!((cámara.pantalla.0.y + 1.0).abs() < f64::EPSILON);
        assert!((cámara.pantalla.3.y - 1.0).abs() < f64::EPSILON);
        assert!((cámara.pantalla.0.z - 2.0).abs() < f64::EPSILON);
        assert!((cámara.pantalla.3.z + 2.0).abs() < f64::EPSILON);
        assert_eq!(cámara.ancho, resolución.0);
        assert_eq!(cámara.alto, resolución.1);
    }












}

