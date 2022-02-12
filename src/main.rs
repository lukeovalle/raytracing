mod camara;
mod geometria;
mod material;
mod modelos;
mod escena;

use geometria::Punto;
use material::{Color, Material};

fn main() {
    let ancho = 300;
    let alto = 200;

    let cámara = camara::Cámara::new(
        &Punto::new(-2.5, 0.0, 0.2),
        0.1,
        120.0,
        (0.0, 0.0, 0.0),
        (ancho, alto)
    );

    let mut escena = escena::Escena::new(&cámara);
//    let modelo = Obj::load(&Path::new("mono.obj")).unwrap();
    let esfera = modelos::Esfera::new(
        &Punto::new(0.0, 0.0, 1.0),
        1.0,
        &Material {
            color_ambiente: Some(Color::new(0.8, 0.2, 0.2)),
            ..Default::default()
        }
    );
    let piso = modelos::Esfera::new(
        &Punto::new(0.0, 0.0, -2000.0),
        2000.0,
        &Material {
            color_ambiente: Some(Color::new(0.6, 0.5, 0.1)),
            ..Default::default()
        }
    );
    let triángulo = modelos::Triángulo::new(
        &Punto::new(0.0, 1.0, 0.0),
        &Punto::new(0.0, 2.0, 0.0),
        &Punto::new(0.0, 1.5, 1.0),
        &Material {
            color_ambiente: Some(Color::new(0.2, 0.1, 0.9)),
            ..Default::default()
        }
    );

    escena.añadir_objeto(&triángulo).unwrap();
    escena.añadir_objeto(&esfera).unwrap();
    escena.añadir_objeto(&piso).unwrap();

    escena.añadir_luz(&escena::Luz::new(&Punto::new(-1.0, -2.0, 2.0))).unwrap();

    let imagen = escena.renderizar();

    imagen.unwrap().save("archivo.bmp").unwrap();
}
