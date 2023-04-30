mod auxiliar;
mod camera;
mod geometry;
mod integrator;
mod material;
mod models;
mod scene;

use geometry::Point;
use material::{Color, Material};

fn main() {
    let width = 300;
    let length = 200;

    let camera = camera::Camera::new(
        &Point::new(-2.5, 0.0, 2.0),
        0.1,
        120.0,
        (0.0, 0.5, 0.0),
        (width, length)
    );

    let mut scene = scene::Scene::new();

    let walls = models::ModelObj::new("cubo.obj").unwrap();
//    let mono = modelos::ModeloObj::new("mono.obj").unwrap();
    let sphere = models::Sphere::new(
        &Point::new(0.0, -2.0, 1.0),
        1.0,
        &Material {
            tipo: material::Type::Lambertian,
            ambient_color: Some(Color::new(0.8, 0.2, 0.2)),
            ..Default::default()
        }
    );

    let mirror = models::Sphere::new(
        &Point::new(0.0, 0.0, 1.0),
        1.0,
        &Material {
            tipo: material::Type::Specular,
            specular_color: Some(Color::new(0.8, 0.8, 0.8)),
            ..Default::default()
        }
    );

    let sphere_2 = models::Sphere::new(
        &Point::new(0.0, 2.0, 1.0),
        1.0,
        &Material {
            tipo: material::Type::Specular,
            specular_color: Some(Color::new(0.8, 0.2, 0.2)),
            ..Default::default()
        }
    );


    let floor = models::Triangle::new(
        &Point::new(-200.0, -100.0, 0.0),
        &Point::new(100.0, 200.0, 0.0),
        &Point::new(100.0, -100.0, 0.0),
        &Material {
            tipo: material::Type::Lambertian,
            ambient_color: Some(Color::new(0.6, 0.5, 0.1)),
            ..Default::default()
        }
    );
    let triangle = models::Triangle::new(
        &Point::new(-1.0, 1.0, 0.01),
        &Point::new(-1.0, 2.0, 0.01),
        &Point::new(0.0, 2.0, 0.01),
        &Material {
            tipo: material::Type::Specular,
            specular_color: Some(Color::new(0.2, 0.1, 0.9)),
            ..Default::default()
        }
    );

    let light = models::Sphere::new(
        &Point::new(0.0, 0.0, 3.0),
        0.8,
        &Material {
            tipo: material::Type::Emitter,
            emitted_color: Some(Color::new(0.6, 0.6, 0.6)),
            ..Default::default()
        }
    );

//    scene.add_shape(&mono).unwrap();
    scene.add_shape(&triangle).unwrap();
    scene.add_shape(&sphere).unwrap();
    scene.add_shape(&mirror).unwrap();
    scene.add_shape(&sphere_2).unwrap();
    scene.add_shape(&floor).unwrap();
    scene.add_shape(&walls).unwrap();

    // esta va a ser una luz
    scene.add_shape(&light).unwrap();

    let integrator = integrator::Integrator::new(
        &camera,
        integrator::IntegratorType::Whitted { depth: 10 }
    );

    let imagen = integrator.render(&scene);
    // let imagen = scene.render();

    imagen.unwrap().save("archivo.bmp").unwrap();
}

