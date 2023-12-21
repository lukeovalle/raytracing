use crate::camera::Camera;
use crate::geometry::Ray;
use crate::parallel::ThreadPool;
use crate::scene::Scene;
use crate::shapes::ShapeOperations;
use crate::spectrum::{SampledSpectrum, SpectrumType};
use enum_dispatch::enum_dispatch;
use image::{ImageBuffer, Pixel, Rgb};
use indicatif::{ProgressBar, ProgressStyle};
use itertools::Itertools;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

type Image = ImageBuffer<Rgb<u8>, Vec<<Rgb<u8> as Pixel>::Subpixel>>;

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
    MonteCarloIntegrator,
    AlbedoIntegrator,
    NormalIntegrator,
}

#[derive(Clone, Debug)]
pub struct MonteCarloIntegrator {
    camera: Camera,
    scene: Scene,
    depth: usize,
    iterations: usize,
}

impl MonteCarloIntegrator {
    pub fn new(camera: &Camera, scene: &Scene, depth: usize, iterations:
    usize) -> Self {
        Self {
            camera: *camera,
            scene: scene.clone(),
            depth,
            iterations,
        }
    }
}

impl SamplerIntegrator for MonteCarloIntegrator {
    fn camera(&self) -> &Camera {
        &self.camera
    }

    fn scene(&self) -> &Scene { &self.scene }

    fn max_depth(&self) -> usize {
        self.depth
    }

    fn total_samples(&self) -> usize {
        self.iterations
    }

    #[allow(non_snake_case)]
    fn incident_light(
        &self,
        ray: &Ray,
        depth: usize
    ) -> SampledSpectrum {
        let mut light = SampledSpectrum::new(0.0);
        // busco el rayo más cercano.
        let mut intersection = match self.scene.intersect_ray(ray) {
            Some(isect) => isect,
            None => return light, // acá debería sumar todas las luces
                                  // que intersecan el rayo
        };

        let normal = intersection.normal();
        let direction_out = intersection.incident_ray().dir();

        // calcular scattering functions


        // Si choqué un objeto emisivo, sumar su luz


        // Sumar la contribución de cada fuente de luz en el punto de
        // intersección. Antes de eso implementar fuentes de luz


        if depth == 0 {
            return SampledSpectrum::new(0.0);
        }

        // Sumar refracción y reflexión especular


        self.scene.shade_point(&intersection, depth)
    }
}

#[derive(Clone, Debug)]
pub struct AlbedoIntegrator {
    camera: Camera,
    scene: Scene,
    iterations: usize,
}

impl AlbedoIntegrator {
    pub fn new(camera: &Camera, scene: &Scene, iterations: usize) -> AlbedoIntegrator {
        AlbedoIntegrator {
            camera: *camera,
            scene: scene.clone(),
            iterations,
        }
    }
}

impl SamplerIntegrator for AlbedoIntegrator {
    fn camera(&self) -> &Camera { &self.camera }

    fn scene(&self) -> &Scene { &self.scene }

    fn max_depth(&self) -> usize { 0 }

    fn total_samples(&self) -> usize { self.iterations }

    fn incident_light(
        &self,
        ray: &Ray,
        _depth: usize
    ) -> SampledSpectrum {
        let mut black = SampledSpectrum::new(0.0);
        // busco el objeto más cercano.
        let mut intersection = match self.scene.intersect_ray(ray) {
            Some(isect) => isect,
            None => return black,
        };

        if let(Some(ambient)) = intersection.model().material().ambient_color {
            return ambient;
        } else if let(Some(emitted)) = intersection.model().material().emitted_color {
            return emitted;
        } else if let(Some(diffuse)) = intersection.model().material().diffused_color {
            return diffuse;
        } else if let(Some(specular)) = intersection.model().material().specular_color {
            return specular;
        }

        black
    }
}

#[derive(Clone, Debug)]
pub struct NormalIntegrator {
    camera: Camera,
    scene: Scene,
    iterations: usize,
}

impl NormalIntegrator {
    pub fn new(camera: &Camera, scene: &Scene, iterations: usize) -> NormalIntegrator {
        NormalIntegrator {
            camera: *camera,
            scene: scene.clone(),
            iterations,
        }
    }
}

impl SamplerIntegrator for NormalIntegrator {
    fn camera(&self) -> &Camera { &self.camera }

    fn scene(&self) -> &Scene { &self.scene }

    fn max_depth(&self) -> usize { 0 }

    fn total_samples(&self) -> usize { self.iterations }

    fn incident_light(
        &self,
        ray: &Ray,
        _depth: usize
    ) -> SampledSpectrum {
        match self.scene.intersect_ray(ray) {
            Some(intersection) => {
                let normalize_normal = |v: f64| {
                    v.mul_add(0.5, 0.5) as f32
                };

                let normal = intersection.normal();

                SampledSpectrum::from_RGB(
                    (normalize_normal(normal.x), normalize_normal(normal.y), normalize_normal(normal.z)),
                    SpectrumType::Reflectance
                )
            }
            None => SampledSpectrum::new(0.0),
        }

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
