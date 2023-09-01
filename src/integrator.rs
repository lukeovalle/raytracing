use crate::camera::Camera;
use crate::geometry::Ray;
use crate::material::{Color, clamp_color, mix_colors};
use crate::scene::Scene;
use image::{ImageBuffer, Rgb, Pixel};
use indicatif::{ProgressBar, ProgressStyle};
use itertools::Itertools;
use enum_dispatch::enum_dispatch;

type Image = ImageBuffer<Rgb<u8>, Vec<<Rgb<u8> as Pixel>::Subpixel>>;

#[enum_dispatch]
pub trait IntegratorRender {
    fn render(self, scene: &Scene) -> Result<Image, anyhow::Error>;
}


#[enum_dispatch(IntegratorRender)]
pub enum Integrator {
    SamplerIntegrator,
    WhittedIntegrator
}

pub struct WhittedIntegrator {
    camera: Camera,
    depth: usize,
}

impl WhittedIntegrator {
    pub fn new(camera: &Camera, depth: usize) -> WhittedIntegrator {
        WhittedIntegrator {
            camera: *camera,
            depth,
        }
    }

    #[allow(non_snake_case)]
    fn whitted_IL(
        &self,
        ray: &Ray,
        scene: &Scene
    ) -> Color {
        if self.depth == 0 {
            return Color::zeros();
        }

        let mut intersection = match scene.intersect_ray(ray) {
            Some(isect) => isect,
            None => return Color::zeros(), // should return a skybox maybe
        };

        if intersection.normal().dot(ray.direction()) > 0.0 {
            // Normal direction goes "inside" object
            intersection.invert_normal()
        } else {
            // Normal goes outside
        }

        scene.shade_point(&intersection, self.depth)
    }
}

fn initialize_progress_bar(width: u64, height: u64) -> Result<ProgressBar, anyhow::Error> {
    let barrita = ProgressBar::new(width * height);

    barrita.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise} ({duration} estimado)] [{wide_bar:.cyan/blue}] {percent}%")?
        .progress_chars("#>-")
    );

    Ok(barrita)
}

impl IntegratorRender for WhittedIntegrator {
    fn render(self, scene: &Scene) -> Result<Image, anyhow::Error> {
        // preprocess();

        let width = self.camera.width() as usize;
        let height = self.camera.height() as usize;

        let mut img = vec![vec![Rgb([0, 0, 0]); height]; width];

        let tile_size = 16;

        // cantidad de tiles, redondea hacia arriba
        let n_tiles = (
            (width + tile_size - 1) / tile_size,
            (height + tile_size - 1) / tile_size
        );

        let barrita =
            initialize_progress_bar(n_tiles.0 as u64, n_tiles.1 as u64)?;
        let mut contador_barrita = 0;

        for (tile_x, tile_y) in (0..n_tiles.0).cartesian_product(0..n_tiles.1) {
            // Cada iteración de esto debería ser en paralelo
            // por ahora no

            // busco bordes del tile
            let (x_0, y_0) = (tile_x * tile_size, tile_y * tile_size);
            let (x_1, y_1) = (
                std::cmp::min(x_0 + tile_size, width),
                std::cmp::min(y_0 + tile_size, height),
            );

            for (i, j) in (x_0..x_1).cartesian_product(y_0..y_1) {
                // Integración de Monte Carlo
                let samples_per_pixel = 300; // parametrizar esto
                let colores: Vec<Color> = (0..samples_per_pixel)
                    .map(|_| {
                        let (v_1, v_2): (f64, f64) =
                            (rand::random(), rand::random());
                        let (v_1, v_2) = (v_1 - 0.5, v_2 - 0.5);

                        let ray =
                            self.camera.get_ray(i as f64 + v_1, j as f64 + v_2);

                        self.whitted_IL(&ray, scene)
                    })
                    .collect();

                let mut color = mix_colors(&colores);
                clamp_color(&mut color);

                // corrección gamma
                img[i][j] = Rgb([
                    (256.0 * color.x.powf(1.0 / 2.2)) as u8,
                    (256.0 * color.y.powf(1.0 / 2.2)) as u8,
                    (256.0 * color.z.powf(1.0 / 2.2)) as u8,
                ]);
            }

            contador_barrita += 1;
            barrita.set_position(contador_barrita);
        }

        let mut buffer_img =
            ImageBuffer::new(self.camera.width(), self.camera.height());

        for (x, y, pixel) in buffer_img.enumerate_pixels_mut() {
            *pixel = img[x as usize][y as usize];
        }

        barrita.finish_with_message("ARchivo guardado en archivo.bmp");

        // save image
        Ok(buffer_img)
    }

}

pub struct SamplerIntegrator {
    camera: Camera,
}

impl SamplerIntegrator {
    fn new(camera: &Camera) -> SamplerIntegrator {
        SamplerIntegrator {
            camera: *camera,
        }
    }
}

impl IntegratorRender for SamplerIntegrator {
    fn render(self, scene: &Scene) -> Result<Image, anyhow::Error> {
        // preprocess();
        // render
        Ok(ImageBuffer::new(self.camera.width(), self.camera.height()))
    }
}

