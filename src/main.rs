mod camara;
mod geometria;
mod material;
mod modelos;
mod escena;

use geometria::Punto;
use material::{Color, Material};
use modelos::{ListaModelos, Triángulo};

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
        &Punto::new(-1.0, 1.0, 0.0),
        &Punto::new(-1.0, 2.0, 0.0),
        &Punto::new(0.0, 2.0, 1.0),
        &Material {
            color_ambiente: Some(Color::new(0.2, 0.1, 0.9)),
            ..Default::default()
        }
    );

    let luz = modelos::Esfera::new(
        &Punto::new(-1.0, -2.0, 1.7),
        0.3,
        &Material {
            color_emitido: Some(Color::new(1.0, 1.0, 1.0)),
            ..Default::default()
        }
    );

    let caja = hacer_paredes(&[
        Punto::new(3.0, -3.0, 3.0),
        Punto::new(-3.0, -3.0, 3.0),
        Punto::new(-3.0, 3.0, 3.0),
        Punto::new(3.0, 3.0, 3.0),
        Punto::new(3.0, -3.0, -3.0),
        Punto::new(-3.0, -3.0, -3.0),
        Punto::new(-3.0, 3.0, -3.0),
        Punto::new(3.0, 3.0, -3.0)],
        &Material {
            color_ambiente: Some(Color::new(0.2, 0.3, 0.4)),
            ..Default::default()
        }
    );
    
    escena.añadir_objeto(&caja).unwrap();

    escena.añadir_objeto(&triángulo).unwrap();
    escena.añadir_objeto(&esfera).unwrap();
    escena.añadir_objeto(&piso).unwrap();

    // esta va a ser una luz
    escena.añadir_objeto(&luz).unwrap();

    let imagen = escena.renderizar();

    imagen.unwrap().save("archivo.bmp").unwrap();
}

// 0 1 2 3 esquinas del techo, contrareloj. 4 5 6 7 esquinas del piso
fn hacer_paredes(esquinas: &[Punto], material: &Material) -> ListaModelos {
    if esquinas.len() < 8 {
        panic!();
    }

    let caras: Vec<_> = [
        [0, 2, 1],
        [0, 3, 2],
        [4, 5, 6],
        [4, 6, 7],
        [0, 4, 1],
        [4, 5, 1],
        [2, 3, 7],
        [2, 7, 6],
        [0, 4, 7],
        [0, 7, 3],
        [1, 6, 5],
        [2, 5, 6]
    ].iter().map(|arr| {
        Triángulo::new(
        &esquinas[arr[0]],
        &esquinas[arr[1]],
        &esquinas[arr[2]],
        material
        )
    }).collect();

    let mut lista = ListaModelos::new();

    for c in caras.iter() {
        lista.añadir_modelo(Box::new(*c));
    }

    lista
}

