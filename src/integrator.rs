use crate::camera::Camera;
use crate::geometry::Ray;
use crate::material::{Color, clamp_color, mix_colors};
use crate::scene::Scene;
use image::{ImageBuffer, Rgb, Pixel};
use indicatif::{ProgressBar, ProgressStyle};


pub enum IntegratorType {
    Whitted { depth: usize },
}
pub struct Integrator {
    camera: Camera,
    integrator: IntegratorType,
}

type Image = ImageBuffer<Rgb<u8>, Vec<<Rgb<u8> as Pixel>::Subpixel>>;

impl Integrator {
    pub fn new(camera: &Camera, integrator: IntegratorType) -> Integrator {
        Integrator {
            camera: *camera,
            integrator,
        }
    }
    pub fn render(self, scene: &Scene) -> Result<Image, anyhow::Error> {
        // preprocess();


        let mut buffer_img =
            ImageBuffer::new(self.camera.width(), self.camera.height());

        let barrita = ProgressBar::new(
            self.camera.width() as u64 * self.camera.height() as u64,
        );

        barrita.set_style(ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise} ({duration} estimado)] [{wide_bar:.cyan/blue}] {percent}%")?
            .progress_chars("#>-")
        );

        // render
        for (x, y, pixel) in buffer_img.enumerate_pixels_mut() {
            // Integración de Monte Carlo
            let samples_per_pixel = 300;
            let colores: Vec<Color> = (0..samples_per_pixel)
                .map(|_| {
                    let (v_1, v_2): (f64, f64) =
                        (rand::random(), rand::random());
                    let (v_1, v_2) = (v_1 - 0.5, v_2 - 0.5);

                    let ray =
                        self.camera.get_ray(x as f64 + v_1, y as f64 + v_2);

                    self.integrator.incident_light(&ray, scene)
                })
            .collect();

            let mut color = mix_colors(&colores);
            clamp_color(&mut color);

            // corrección gamma
            *pixel = Rgb([
                (256.0 * color.x.powf(1.0 / 2.2)) as u8,
                (256.0 * color.y.powf(1.0 / 2.2)) as u8,
                (256.0 * color.z.powf(1.0 / 2.2)) as u8,
            ]);

            barrita.set_position(
                y as u64 * self.camera.width() as u64 + (x + 1) as u64,
            );
        }

        barrita.finish_with_message("ARchivo guardado en archivo.bmp");

        // save image
        Ok(buffer_img)
    }
}

impl IntegratorType {
    fn incident_light(&self, ray: &Ray, scene: &Scene) -> Color {
        match self {
            IntegratorType::Whitted { depth } =>
                self.whitted_IL(ray, scene, *depth)
        }
    }

    #[allow(non_snake_case)]
    fn whitted_IL(
        &self,
        ray: &Ray,
        scene: &Scene,
        depth: usize
    ) -> Color {
        if depth == 0 {
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

        scene.shade_point(&intersection, depth)
    }
}

