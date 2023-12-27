use super::integrator::SamplerIntegrator;
use crate::camera::Camera;
use crate::geometry::Ray;
use crate::scene::Scene;
use crate::spectrum::SampledSpectrum;

#[derive(Clone, Debug)]
pub struct RandomWalkIntegrator {
    camera: Camera,
    scene: Scene,
    depth: usize,
    iterations: usize,
}

impl RandomWalkIntegrator {
    pub fn new(
        camera: &Camera,
        scene: &Scene,
        depth: usize,
        iterations: usize,
    ) -> Self {
        Self {
            camera: *camera,
            scene: scene.clone(),
            depth,
            iterations,
        }
    }
}

impl SamplerIntegrator for RandomWalkIntegrator {
    fn camera(&self) -> &Camera {
        &self.camera
    }

    fn scene(&self) -> &Scene {
        &self.scene
    }

    fn max_depth(&self) -> usize {
        self.depth
    }

    fn total_samples(&self) -> usize {
        self.iterations
    }

    #[allow(non_snake_case)]
    fn incident_light(&self, ray: &Ray, depth: usize) -> SampledSpectrum {
        let light = SampledSpectrum::new(0.0);
        // busco el rayo más cercano.
        let intersection = match self.scene.intersect_ray(ray) {
            Some(isect) => isect,
            None => return light, // acá debería sumar todas las luces
                                  // que intersecan el rayo
        };

        let _normal = intersection.normal();
        let _direction_out = intersection.incident_ray().dir();

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
