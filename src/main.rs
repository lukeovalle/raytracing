mod camara;
mod geometria;
mod modelos;
mod escena;

use geometria::Punto;

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
    let esfera = modelos::Esfera::new(&Punto::new(0.0, 0.0, 1.0), 1.0);
    let piso = modelos::Esfera::new(&Punto::new(0.0, 0.0, -2000.0), 2000.0);
    let triángulo = geometria::Triángulo::new(
        &Punto::new(0.0, 1.0, 0.0),
        &Punto::new(0.0, 2.0, 0.0),
        &Punto::new(0.0, 1.5, 1.0)
    );

    escena.añadir_objeto(&triángulo).unwrap();
    escena.añadir_objeto(&esfera).unwrap();
    escena.añadir_objeto(&piso).unwrap();

    escena.añadir_luz(&escena::Luz::new(&Punto::new(-1.0, -2.0, 2.0))).unwrap();

    let imagen = escena.renderizar();

    imagen.unwrap().save("archivo.bmp").unwrap();
}
