use super::albedo::AlbedoIntegrator;
use super::auxiliar::{Image, initialize_progress_bar};
use super::normal::NormalIntegrator;
use super::random_walk::RandomWalkIntegrator;
use crate::camera::Camera;
use crate::parallel::ThreadPool;
use crate::scene::Scene;
use crate::geometry::Ray;
use crate::spectrum::SampledSpectrum;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicUsize, Ordering};
use enum_dispatch::enum_dispatch;
use image::{ImageBuffer, Rgb};
use itertools::Itertools;

#[enum_dispatch]
pub trait SamplerIntegrator: Sync {
    /// Reference to the camera.
    fn camera(&self) -> &Camera;

    /// Reference to the scene.
    fn scene(&self) -> &Scene;

    /// Depth of the recursion.
    fn max_depth(&self) -> usize;

    /// Number of samples.
    fn total_samples(&self) -> usize;

    fn render(&self) -> Result<Image, anyhow::Error>
    {
        // preprocess();

        let width = self.camera().width() as usize;
        let height = self.camera().height() as usize;

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

        let tiles = (0..n_tiles.0).cartesian_product(0..n_tiles.1);


        let mut thread_pool = ThreadPool::new();

        for (tile_x, tile_y) in tiles {
            let img_clone = img.clone();
            let contador_iter_clone = contador_iter.clone();
            let barrita_clone = barrita.clone();

            thread_pool.add_task(move || {
                // busco bordes del tile
                let (x_0, y_0) = (tile_x * tile_size, tile_y * tile_size);
                let (x_1, y_1) = (
                    std::cmp::min(x_0 + tile_size, width),
                    std::cmp::min(y_0 + tile_size, height),
                );

                for (x, y) in (x_0..x_1).cartesian_product(y_0..y_1) {
                    // Integración de Monte Carlo
                    let colores: Vec<SampledSpectrum> = (0..self.total_samples())
                        .map(|sample_index| {
                            self.evaluate_pixel_sample(
                                (x, y),
                                sample_index
                            )
                        })
                        .collect();

                    let color = colores.iter()
                        .fold(SampledSpectrum::new(0.0), |acc, x| acc + *x);
                    let color = &color / self.total_samples() as f32;
                    let (r, g, b) = color.to_RGB();
                    let r = r.clamp(0.0, 1.0);
                    let g = g.clamp(0.0, 1.0);
                    let b = b.clamp(0.0, 1.0);

                    // corrección gamma
                    img_clone.lock().unwrap()[x][y] = Rgb([
                        (256.0 * r.powf(1.0 / 2.2)) as u8,
                        (256.0 * g.powf(1.0 / 2.2)) as u8,
                        (256.0 * b.powf(1.0 / 2.2)) as u8,
                    ]);
                }

                contador_iter_clone.fetch_add(1, Ordering::SeqCst);
                barrita_clone.inc(1);
            });
        }


        thread_pool.run(|| {
            'wait: loop {
                let i = contador_iter.load(Ordering::SeqCst);
                if i >= n_tiles.0 * n_tiles.1 {
                    break 'wait;
                }

                std::thread::sleep(std::time::Duration::from_millis(60));
            }
        });

        let mut buffer_img =
            ImageBuffer::new(self.camera().width(), self.camera().height());

        for (x, y, pixel) in buffer_img.enumerate_pixels_mut() {
            *pixel = img.lock().unwrap()[x as usize][y as usize];
        }

        barrita.finish_with_message("Finalizado.");

        // save image
        Ok(buffer_img)
    }


    /// Takes a sample at the given pixel and returns the sampled color.
    fn evaluate_pixel_sample(
        &self,
        (x, y): (usize, usize),
        _sample_index: usize
    ) -> SampledSpectrum {
        // todo: cuando cambie el SampledSpectrum por SampledWavelenghts tengo
        // todo: que generar acá las longitudes de onda muestreadas.
        let random_sample = |val: usize| val as f64 + rand::random::<f64>() - 0.5;

        let (v_1, v_2): (f64, f64) = (random_sample(x), random_sample(y));

        let ray = self.camera().get_ray(v_1, v_2);

        self.incident_light(&ray, self.max_depth())
    }

    ///Returns the incident light coming from the ray.
    fn incident_light(
        &self,
        ray: &Ray,
        depth: usize
    ) -> SampledSpectrum;
}

#[enum_dispatch(SamplerIntegrator)]
#[derive(Clone, Debug)]
pub enum Integrator {
    RandomWalkIntegrator,
    AlbedoIntegrator,
    NormalIntegrator,
}
