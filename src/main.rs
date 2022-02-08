use image::ImageBuffer;
use image::Rgb;
use obj::Obj;

fn main() {
    let ancho = 800;
    let alto = 600;

    let cámara = Cámara::new(
        Punto::new(-10.0, 0.0, 0.0),
        0.1,
        120,
        (0.0, 0.0, 0.0),
        (ancho, alto)
    );

    let escena = Escena::new(&cámara);
//    let modelo = Obj::load(&Path::new("mono.obj")).unwrap();
    let esfera = Esfera::new(Punto(1.0, 1.0, 1.0), 1.0);

    escena.añadir_objeto(esfera);

    let imagen = escena.renderizar();

    imagen.save("archivo.bmp").unwrap();
}
