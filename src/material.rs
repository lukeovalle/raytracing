use crate::spectrum::{SampledSpectrum, SpectrumType};
use wavefront_obj::mtl;

#[derive(Clone, Copy, Debug)]
pub enum Type {
    Emitter,
    Lambertian,
    Specular,
}

#[derive(Clone, Copy, Debug)]
pub struct Material {
    //nombre: String,   // no necesito nombre creo
    pub tipo: Type,
    pub ambient_color: Option<SampledSpectrum>, // el color base
    pub emitted_color: Option<SampledSpectrum>, // si emite luz, tira este color
    pub diffused_color: Option<SampledSpectrum>, // para la reflexión difusa (rayos
    // reflejados difusos)
    pub specular_color: Option<SampledSpectrum>, // para los rayos reflejados
    pub specular_coefficient: Option<f64>,       // para la reflexión especular
    // creo, va de 0 a 1000 parece
    pub optical_density: Option<f64>, // el coeficiente de refracción
}

impl Default for Material {
    fn default() -> Self {
        Material {
            tipo: Type::Emitter,
            ambient_color: None,
            emitted_color: None,
            diffused_color: None,
            specular_color: None,
            specular_coefficient: None,
            optical_density: None,
        }
    }
}

impl From<&mtl::Material> for Material {
    fn from(mat: &mtl::Material) -> Self {
        Material {
            tipo: Type::Lambertian, // después ver que hacer con esto
            ambient_color: Some(create_spectrum_from_mtl(&mat.color_ambient)),
            emitted_color: mat
                .color_emissive
                .map(|c| create_spectrum_from_mtl(&c)),
            diffused_color: Some(create_spectrum_from_mtl(&mat.color_diffuse)),
            specular_color: Some(create_spectrum_from_mtl(&mat.color_specular)),
            specular_coefficient: Some(mat.specular_coefficient),
            optical_density: mat.optical_density,
        }
    }
}

#[inline]
fn create_spectrum_from_mtl(color: &mtl::Color) -> SampledSpectrum {
    SampledSpectrum::from_RGB(
        (color.r as f32, color.g as f32, color.b as f32),
        SpectrumType::Reflectance,
    )
}
