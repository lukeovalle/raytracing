mod auxiliar;
mod camera;
mod geometry;
mod integrators;
mod material;
mod parallel;
mod scene;
mod scene_config;
mod shapes;
mod spectrum;

use std::env;

use integrators::AlbedoIntegrator;
use integrators::NormalIntegrator;
use integrators::RandomWalkIntegrator;
use integrators::{Integrator, SamplerIntegrator};

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
    init();

    if let Err(e) = program() {
        eprintln!("{e:?}");
    }
}

/// Inicializa cosas static
fn init() {
    spectrum::SampledSpectrum::init();
}

fn program() -> Result<(), anyhow::Error> {
    let (scene_description, output) = match parse_args() {
        Some((scene_description, output)) => (scene_description, output),
        None => return Ok(()),
    };

    let input_toml = scene_config::parse_file(&scene_description)?;

    let camera = scene_config::parse_camera(&input_toml)?;

    let scene = scene_config::parse_scene(&input_toml)?;

    // Primero genero una imagen de los colores
    let integrator: Integrator =
        AlbedoIntegrator::new(&camera, &scene, 20).into();

    let imagen = integrator.render()?;

    imagen.save("output-albedo.bmp")?;
    println!("Imagen guardada en output-albedo.bmp.");

    // Genero una imagen de las normales
    let integrator: Integrator =
        NormalIntegrator::new(&camera, &scene, 20).into();

    let imagen = integrator.render()?;

    imagen.save("output-normal.bmp")?;
    println!("Imagen guardada en output-normal.bmp.");

    // todo: que el integrator reciba el número de muestras.
    let integrator: Integrator =
        RandomWalkIntegrator::new(&camera, &scene, 10, 100).into();

    let imagen = integrator.render()?;

    imagen.save(&output)?;
    println!("Imagen guardada en \"{output}\".");

    Ok(())
}
