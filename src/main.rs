mod auxiliares;
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
        &Punto::new(-2.5, 0.0, 2.0),
        0.1,
        120.0,
        (0.0, 0.5, 0.0),
        (ancho, alto)
    );

    let mut escena = escena::Escena::new(&cámara);

    let paredes = modelos::ModeloObj::new("cubo.obj").unwrap();
//    let mono = modelos::ModeloObj::new("mono.obj").unwrap();
    let esfera = modelos::Esfera::new(
        &Punto::new(0.0, -2.0, 1.0),
        1.0,
        &Material {
            tipo: material::Tipo::Lambertiano,
            color_ambiente: Some(Color::new(0.8, 0.2, 0.2)),
            ..Default::default()
        }
    );

    let espejo = modelos::Esfera::new(
        &Punto::new(0.0, 0.0, 1.0),
        1.0,
        &Material {
            tipo: material::Tipo::Especular,
            color_especular: Some(Color::new(0.8, 0.8, 0.8)),
            ..Default::default()
        }
    );

    let esfera_2 = modelos::Esfera::new(
        &Punto::new(0.0, 2.0, 1.0),
        1.0,
        &Material {
            tipo: material::Tipo::Especular,
            color_especular: Some(Color::new(0.8, 0.2, 0.2)),
            ..Default::default()
        }
    );


    let piso = modelos::Triángulo::new(
        &Punto::new(-200.0, -100.0, 0.0),
        &Punto::new(100.0, 200.0, 0.0),
        &Punto::new(100.0, -100.0, 0.0),
        &Material {
            tipo: material::Tipo::Lambertiano,
            color_ambiente: Some(Color::new(0.6, 0.5, 0.1)),
            ..Default::default()
        }
    );
    let triángulo = modelos::Triángulo::new(
        &Punto::new(-1.0, 1.0, 0.01),
        &Punto::new(-1.0, 2.0, 0.01),
        &Punto::new(0.0, 2.0, 0.01),
        &Material {
            tipo: material::Tipo::Especular,
            color_especular: Some(Color::new(0.2, 0.1, 0.9)),
            ..Default::default()
        }
    );

    let luz = modelos::Esfera::new(
        &Punto::new(0.0, 0.0, 3.0),
        0.8,
        &Material {
            tipo: material::Tipo::Emisor,
            color_emitido: Some(Color::new(0.6, 0.6, 0.6)),
            ..Default::default()
        }
    );

//    escena.añadir_objeto(&mono).unwrap();
    escena.añadir_objeto(&triángulo).unwrap();
    escena.añadir_objeto(&esfera).unwrap();
    escena.añadir_objeto(&espejo).unwrap();
    escena.añadir_objeto(&esfera_2).unwrap();
    escena.añadir_objeto(&piso).unwrap();
    escena.añadir_objeto(&paredes).unwrap();

    // esta va a ser una luz
    escena.añadir_objeto(&luz).unwrap();

    let imagen = escena.renderizar();

    imagen.unwrap().save("archivo.bmp").unwrap();
}

