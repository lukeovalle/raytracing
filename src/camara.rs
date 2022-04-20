use crate::geometria::{Punto, Rayo, Rectángulo};

#[derive(Clone, Copy, Debug)]
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
        foco: &Punto,
        distancia_focal: f64,
        campo_de_visión: f64,
        inclinación: (f64, f64, f64),
        resolución: (u32, u32)
    ) -> Cámara {
        // p_1 es la esquina de arriba a la izquierda
        // p_2 es la esquina de arriba a la derecha
        // p_3 es la esquina de abajo a la izquierda
        // p_4 es la esquina de abajo a la derecha

        let delta_y = distancia_focal * (campo_de_visión / 2.0 * std::f64::consts::PI / 180.0 ).tan();
        let delta_z = delta_y * resolución.1 as f64 / resolución.0 as f64;

        let rot = nalgebra::Rotation3::from_euler_angles(
            inclinación.0,
            inclinación.1,
            inclinación.2
        );
        let tras = nalgebra::Translation3::new(foco.x, foco.y, foco.z);

        let p_1 = tras * rot * Punto::new(distancia_focal, -delta_y,  delta_z);
        let p_2 = tras * rot * Punto::new(distancia_focal,  delta_y,  delta_z);
        let p_3 = tras * rot * Punto::new(distancia_focal, -delta_y, -delta_z);
        let p_4 = tras * rot * Punto::new(distancia_focal,  delta_y, -delta_z);


        Cámara {
            foco: foco.clone(),
            pantalla: Rectángulo(p_1, p_2, p_3, p_4),
            ancho: resolución.0,
            alto: resolución.1
        }
    }

    /*
    pub fn foco(&self) -> Punto {
        self.foco
    }

    pub fn pantalla(&self) -> &Rectángulo {
        &self.pantalla
    }
    */

    pub fn ancho(&self) -> u32 {
        self.ancho
    }

    pub fn alto(&self) -> u32 {
        self.alto
    }

    pub fn lanzar_rayo(&self, i: f64, j: f64) -> Rayo {
        // Me guié con un paint para sacar esta lógica
        // vec_derecha es el vector entre la esquina derecha y la esquina izquierda,
        // escalado por el i en relación con el ancho (el vector es máximo en el último
        // pixel y es nulo en el primer pixel)
        // vec_abajo lo mismo pero de abajo para arriba
        // con eso creo un punto que va de la esquina superior izquierda, hacia abajo a la
        // derecha.
        // el rayo parte del foco y va en dirección punto-foco
        let vec_derecha = (self.pantalla.1 - self.pantalla.0) * i / self.ancho as f64;
        let vec_abajo = (self.pantalla.2 - self.pantalla.0) * j / self.alto as f64;
        let punto = self.pantalla.0 + vec_derecha + vec_abajo;

        Rayo::new(
            &self.foco,
            &(punto - self.foco)
        )

    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn crear_cámara_sin_rotación() {
        let foco = Punto::new(0.0, 0.0, 0.0);
        let distancia_focal = 1.0;
        let campo_de_visión = 90.0; // la tangente de 45 es 1
        let resolución = (500, 1000); // relación de aspecto 1:2

        let cámara = Cámara::new(
            &foco,
            distancia_focal,
            campo_de_visión,
            (0.0, 0.0, 0.0),
            resolución
        );

        assert!((cámara.foco.x - foco.x).abs() < 1e-10);
        assert!((cámara.foco.y - foco.y).abs() < 1e-10);
        assert!((cámara.foco.z - foco.z).abs() < 1e-10);
        assert!((cámara.pantalla.0.x - distancia_focal).abs() < 1e-10);
        assert!((cámara.pantalla.3.x - distancia_focal).abs() < 1e-10);
        assert!((cámara.pantalla.0.y + 1.0).abs() < 1e-10);
        assert!((cámara.pantalla.3.y - 1.0).abs() < 1e-10);
        assert!((cámara.pantalla.0.z - 2.0).abs() < 1e-10);
        assert!((cámara.pantalla.3.z + 2.0).abs() < 1e-10);
        assert_eq!(cámara.ancho, resolución.0);
        assert_eq!(cámara.alto, resolución.1);
    }

    #[test]
    fn crear_cámara_con_rotación() {
        let foco = Punto::new(0.0, 0.0, 0.0);
        let distancia_focal = 1.0;
        let campo_de_visión = 90.0;
        let resolución = (500, 1000);

        let cámara = Cámara::new(
            &foco,
            distancia_focal,
            campo_de_visión,
            (std::f64::consts::PI/2.0, 0.0, 0.0),
            resolución
        );

        dbg!(cámara);

        assert!((cámara.foco.x - foco.x).abs() < 1e-10);
        assert!((cámara.foco.y - foco.y).abs() < 1e-10);
        assert!((cámara.foco.z - foco.z).abs() < 1e-10);
        assert!((cámara.pantalla.0.x - distancia_focal).abs() < 1e-10);
        assert!((cámara.pantalla.3.x - distancia_focal).abs() < 1e-10);
        assert!((cámara.pantalla.0.y + 2.0).abs() < 1e-10);
        assert!((cámara.pantalla.3.y - 2.0).abs() < 1e-10);
        assert!((cámara.pantalla.0.z + 1.0).abs() < 1e-10);
        assert!((cámara.pantalla.3.z - 1.0).abs() < 1e-10);
        assert_eq!(cámara.ancho, resolución.0);
        assert_eq!(cámara.alto, resolución.1);
    }

    #[test]
    fn lanzar_rayo() {
        let cámara = Cámara::new(
            &Punto::new(0.0, 0.0, 0.0), // foco
            1.0,                        // distancia_focal
            90.0,                       // campo_de_visión
            (0.0, 0.0, 0.0),            // rotación
            (100, 100)                  // ancho, alto
        );

        let rayo = cámara.lanzar_rayo(0.0, 0.0);

        assert!(rayo.origen().x.abs() < 1e-10);
        assert!(rayo.origen().y.abs() < 1e-10);
        assert!(rayo.origen().z.abs() < 1e-10);
        assert!((rayo.dirección().x - 1.0).abs() < 1e-10);
        assert!((rayo.dirección().y + 1.0).abs() < 1e-10);
        assert!((rayo.dirección().z - 1.0).abs() < 1e-10);

        // Pruebo en otro pixel
        let rayo = cámara.lanzar_rayo(50.0, 50.0);
        dbg!(&rayo);

        assert!((rayo.dirección().x - 1.0).abs() < 1e-10);
        assert!(rayo.dirección().y.abs() < 1e-10);
        assert!(rayo.dirección().z.abs() < 1e-10);

    }
}

