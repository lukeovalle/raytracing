use crate::camera::Camera;
use crate::geometry::Ray;
use crate::parallel::parallel_for;
use crate::scene::Scene;
use crate::spectrum::SampledSpectrum;
use enum_dispatch::enum_dispatch;
use image::{ImageBuffer, Pixel, Rgb};
use indicatif::{ProgressBar, ProgressStyle};
use itertools::Itertools;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

type Image = ImageBuffer<Rgb<u8>, Vec<<Rgb<u8> as Pixel>::Subpixel>>;

#[enum_dispatch]
pub trait IntegratorRender {
    fn render(self, scene: &Scene) -> Result<Image, anyhow::Error>;
}

#[enum_dispatch(IntegratorRender)]
pub enum Integrator {
    SamplerIntegrator,
    WhittedIntegrator,
}

#[derive(Clone, Copy)]
pub struct SamplerIntegrator {
    camera: Camera,
    depth: usize,
}

impl SamplerIntegrator {
    pub fn new(camera: &Camera, depth: usize) -> SamplerIntegrator {
        SamplerIntegrator{
            camera: *camera,
            depth,
        }
    }

    #[allow(non_snake_case)]
    fn incident_light(
        &self,
        ray: &Ray,
        scene: &Scene,
        depth: usize
    ) -> SampledSpectrum {
        let mut light = SampledSpectrum::new(0.0);
        // busco el rayo más cercano.
        let mut intersection = match scene.intersect_ray(ray) {
            Some(isect) => isect,
            None => return light, // acá debería sumar todas las luces
                                           // que intersecan el rayo
        };

        let normal = intersection.normal();
        let direction_out = intersection.incident_ray().direction();

        if depth == 0 {
            return SampledSpectrum::new(0.0);
        }

        scene.shade_point(&intersection, depth)
    }
}

impl IntegratorRender for SamplerIntegrator {
    fn render(self, scene: &Scene) -> Result<Image, anyhow::Error> {
        // preprocess();

        let width = self.camera.width() as usize;
        let height = self.camera.height() as usize;

        let img =
            Arc::new(Mutex::new(vec![vec![Rgb([0, 0, 0]); height]; width]));

        let tile_size = 16;

        // cantidad de tiles, redondea hacia arriba
        let n_tiles = (
            (width + tile_size - 1) / tile_size,
            (height + tile_size - 1) / tile_size,
        );

        let barrita = initialize_progress_bar((n_tiles.0 * n_tiles.1) as u64)?;
        let contador_iter = Arc::new(AtomicUsize::new(0));

        let tiles = (0..n_tiles.0).cartesian_product(0..n_tiles.1).collect();

        let img_clone = img.clone();
        let contador_iter_clone = contador_iter.clone();
        let scene_clone = scene.clone();
        let barrita_clone = barrita.clone();
        parallel_for(8, tiles, move |(tile_x, tile_y)| {
            let ref_img = img_clone.clone();
            let contador = contador_iter_clone.clone();

            // busco bordes del tile
            let (x_0, y_0) = (tile_x * tile_size, tile_y * tile_size);
            let (x_1, y_1) = (
                std::cmp::min(x_0 + tile_size, width),
                std::cmp::min(y_0 + tile_size, height),
            );

            for (i, j) in (x_0..x_1).cartesian_product(y_0..y_1) {
                // Integración de Monte Carlo
                let samples_per_pixel = 300; // parametrizar esto
                let colores: Vec<SampledSpectrum> = (0..samples_per_pixel)
                    .map(|_| {
                        let (v_1, v_2): (f64, f64) =
                            (rand::random(), rand::random());
                        let (v_1, v_2) = (v_1 - 0.5, v_2 - 0.5);
                        let (v_1, v_2) = (i as f64 + v_1, j as f64 + v_2);

                        let ray = self.camera.get_ray(v_1, v_2);

                        self.incident_light(&ray, &scene_clone, self.depth)
                    })
                    .collect();

                let color = colores.iter()
                    .fold(SampledSpectrum::new(0.0), |acc, x| acc + *x);
                let color = &color / samples_per_pixel as f32;
                let (r, g, b) = color.to_RGB();
                let r = r.clamp(0.0, 1.0);
                let g = g.clamp(0.0, 1.0);
                let b = b.clamp(0.0, 1.0);

                // corrección gamma
                ref_img.lock().unwrap()[i][j] = Rgb([
                    (256.0 * r.powf(1.0 / 2.2)) as u8,
                    (256.0 * g.powf(1.0 / 2.2)) as u8,
                    (256.0 * b.powf(1.0 / 2.2)) as u8,
                ]);
            }

            contador.fetch_add(1, Ordering::SeqCst);
            barrita_clone.inc(1);
        });

        'wait: loop {
            let i = contador_iter.load(Ordering::SeqCst);
            if i >= n_tiles.0 * n_tiles.1 {
                break 'wait;
            }

            std::thread::sleep(std::time::Duration::from_millis(60));
        }

        let mut buffer_img =
            ImageBuffer::new(self.camera.width(), self.camera.height());

        for (x, y, pixel) in buffer_img.enumerate_pixels_mut() {
            *pixel = img.lock().unwrap()[x as usize][y as usize];
        }

        barrita.finish_with_message("Finalizado.");

        // save image
        Ok(buffer_img)
    }
}

pub struct WhittedIntegrator {
    camera: Camera,
}

impl WhittedIntegrator {
    pub fn new(camera: &Camera) -> WhittedIntegrator {
        WhittedIntegrator { camera: *camera }
    }
}

impl IntegratorRender for WhittedIntegrator {
    fn render(self, scene: &Scene) -> Result<Image, anyhow::Error> {
        // preprocess();
        // render

        // compute scattering functions (tengo que ver que es esto)


        // sumar luz emitida por el objeto


        // sumar la luz de todas las fuentes de luz (implementar luces antes)


        // reflección y refracción

        /*
        if intersection.normal().dot(ray.direction()) > 0.0 {
            // Normal direction goes "inside" object
            intersection.invert_normal()
        } else {
            // Normal goes outside
        }
        */


        Ok(ImageBuffer::new(self.camera.width(), self.camera.height()))
    }
}

fn initialize_progress_bar(size: u64) -> Result<ProgressBar, anyhow::Error> {
    let barrita = ProgressBar::new(size);

    barrita.set_style(
        ProgressStyle::default_bar()
            .template(
                "{spinner:.green} [{elapsed_precise} ({duration} estimado)] \
            {msg} [{wide_bar:.cyan/blue}] \
            [{human_pos}/{human_len} tiles] {percent}%",
            )?
            .progress_chars("#>-"),
    );

    Ok(barrita)
}


