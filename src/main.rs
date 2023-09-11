mod auxiliar;
mod camera;
mod geometry;
mod integrator;
mod material;
mod models;
mod parallel;
mod scene;
mod scene_config;

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
        eprintln!("{e:?}");
    }
}

fn program() -> Result<(), anyhow::Error> {
    let (scene_description, output) = match parse_args() {
        Some((scene_description, output)) => (scene_description, output),
        None => return Ok(()),
    };

    let input_toml = scene_config::parse_file(&scene_description)?;

    let camera = scene_config::parse_camera(&input_toml)?;

    let scene = scene_config::parse_scene(&input_toml)?;

    // todo: que el integrator reciba el número de muestras.
    let integrator: Integrator = WhittedIntegrator::new(&camera, 10).into();

    let imagen = integrator.render(&scene)?;

    imagen.save(&output)?;
    println!("Imagen guardada en \"{output}\".");

    Ok(())
}
