use super::integrator::SamplerIntegrator;
use crate::camera::Camera;
use crate::geometry::Ray;
use crate::scene::Scene;
use crate::spectrum::{SampledSpectrum, SpectrumType};

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
