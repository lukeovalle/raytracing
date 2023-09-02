mod auxiliar;
mod camera;
mod geometry;
mod integrator;
mod material;
mod models;
mod parallel;
mod scene;

use geometry::Point;
use integrator::{Integrator, IntegratorRender};
use material::{Color, Material};
use models::Model;

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
    scene.add_shape(&Model::from(triangle)).unwrap();
    scene.add_shape(&Model::from(sphere)).unwrap();
    scene.add_shape(&Model::from(mirror)).unwrap();
    scene.add_shape(&Model::from(sphere_2)).unwrap();
    scene.add_shape(&Model::from(floor)).unwrap();
    scene.add_shape(&Model::from(walls)).unwrap();

    // esta va a ser una luz
    scene.add_shape(&Model::from(light)).unwrap();

    let integrator = integrator::WhittedIntegrator::new(&camera, 10);
    let integrator = Integrator::from(integrator);



    let imagen = integrator.render(&scene);
    // let imagen = scene.render();

    imagen.unwrap().save("archivo.bmp").unwrap();
}

