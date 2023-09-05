mod auxiliar;
mod camera;
mod geometry;
mod integrator;
mod material;
mod models;
mod parallel;
mod scene;

use integrator::{Integrator, IntegratorRender, WhittedIntegrator};
use std::env;

fn print_help() {
    println!("Uso: raytracer [scene.toml] [file.bmp]");
}

fn parse_args() -> Option<(String, String)> {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() == 1 && vec!["-h", "--help"].contains(&args[0].as_str()) {
        print_help();
        return None;        
    }

    let mut output = "render.bmp".to_string();
    let mut scene = "scene.toml".to_string();

    for arg in args {
        if arg.ends_with(".toml") {
            scene = arg
        } else if arg.ends_with(".bmp") {
            output = arg
        } else {
           print_help();
           return None;
        }
    }

    Some((scene, output))
}

fn main() {
    if let Err(e) = program() {
        eprintln!("{:?}", e);
    }
}

fn program() -> Result<(), anyhow::Error> {
    let (scene_description, output) = match parse_args() {
        Some((scene_description, output)) => (scene_description, output),
        None => {
            return Ok(())
        }
    };

    let scene_description = auxiliar::read_file(&scene_description)?
        .parse::<toml::Table>()?;
    let camera_description = &scene_description.get("camera")
        .map(|c| c.as_table()).flatten()
        .ok_or(anyhow::anyhow!("No se ha especificado la cámara"))?;
    let scene_description = &scene_description.get("scene")
        .map(|s| s.as_array()).flatten()
        .ok_or(anyhow::anyhow!("No se ha especificado la escena"))?;

    let camera = camera::Camera::from_toml(camera_description)?;

    let scene = scene::Scene::from_toml(&scene_description)?;


//    let mono = modelos::ModeloObj::new("mono.obj").unwrap();
//    scene.add_shape(&mono).unwrap();

    let integrator: Integrator = WhittedIntegrator::new(&camera, 10).into();

    let imagen = integrator.render(&scene);
    // let imagen = scene.render();

    imagen.unwrap().save(&output).unwrap();
    println!("Imagen guardada en {}", output);

    Ok(())
}

