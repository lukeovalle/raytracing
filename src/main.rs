mod camara;
mod geometria;
mod objetos;
mod escena;

use geometria::Punto;

fn main() {
    let ancho = 1920;
    let alto = 1080;

    let cámara = camara::Cámara::new(
        &Punto::new(-2.0, 0.0, 0.0),
        0.1,
        120.0,
        (0.0, 0.0, 0.0),
        (ancho, alto)
    );

    let mut escena = escena::Escena::new(&cámara);
//    let modelo = Obj::load(&Path::new("mono.obj")).unwrap();
    let esfera = objetos::Esfera::new(&Punto::new(0.0, 0.0, 0.0), 1.0);

    escena.añadir_objeto(&esfera);

    let imagen = escena.renderizar();

    imagen.unwrap().save("archivo.bmp").unwrap();
}
