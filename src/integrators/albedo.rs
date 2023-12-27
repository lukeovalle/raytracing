use super::integrator::SamplerIntegrator;
use crate::camera::Camera;
use crate::geometry::Ray;
use crate::scene::Scene;
use crate::shapes::ShapeOperations;
use crate::spectrum::SampledSpectrum;

#[derive(Clone, Debug)]
pub struct AlbedoIntegrator {
    camera: Camera,
    scene: Scene,
    iterations: usize,
}

impl AlbedoIntegrator {
    pub fn new(
        camera: &Camera,
        scene: &Scene,
        iterations: usize,
    ) -> AlbedoIntegrator {
        AlbedoIntegrator {
            camera: *camera,
            scene: scene.clone(),
            iterations,
        }
    }
}

impl SamplerIntegrator for AlbedoIntegrator {
    fn camera(&self) -> &Camera {
        &self.camera
    }

    fn scene(&self) -> &Scene {
        &self.scene
    }

    fn max_depth(&self) -> usize {
        0
    }

    fn total_samples(&self) -> usize {
        self.iterations
    }

    fn incident_light(&self, ray: &Ray, _depth: usize) -> SampledSpectrum {
        let black = SampledSpectrum::new(0.0);
        // busco el objeto más cercano.
        let intersection = match self.scene.intersect_ray(ray) {
            Some(isect) => isect,
            None => return black,
        };

        if let Some(ambient) = intersection.model().material().ambient_color {
            return ambient;
        } else if let Some(emitted) =
            intersection.model().material().emitted_color
        {
            return emitted;
        } else if let Some(diffuse) =
            intersection.model().material().diffused_color
        {
            return diffuse;
        } else if let Some(specular) =
            intersection.model().material().specular_color
        {
            return specular;
        }

        black
    }
}
