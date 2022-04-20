use nalgebra::Vector3;
use wavefront_obj::mtl;

pub type Color = Vector3<f64>;

#[derive(Clone, Copy, Debug)]
pub struct Material {
//    nombre: String,   // no necesito nombre creo
    pub color_ambiente: Option<Color>,      // el color base
    pub color_emitido: Option<Color>,       // si emite luz, tira este color
    pub color_difuso: Option<Color>,        // para la reflección difusa (rayos reflejados difusos)
    pub color_especular: Option<Color>,     // para los rayos reflejados
    pub exponente_especular: Option<f64>,   // para la reflección especular creo, va de 0 a 1000 parece
    pub densidad_óptica: Option<f64> // el coeficiente de refracción
}

impl Default for Material {
    fn default() -> Self {
        Material {
            color_ambiente: None,
            color_emitido: None,
            color_difuso: None,
            color_especular: None,
            exponente_especular: None,
            densidad_óptica: None
        }
    }
}

impl From<&mtl::Material> for Material {
    fn from(mat: &mtl::Material) -> Self {
        Material {
            color_ambiente: Some(crear_color_desde_mtl(&mat.color_ambient)),
            color_emitido: mat.color_emissive.map(|c| crear_color_desde_mtl(&c)),
            color_difuso: Some(crear_color_desde_mtl(&mat.color_diffuse)),
            color_especular: Some(crear_color_desde_mtl(&mat.color_specular)),
            exponente_especular: Some(mat.specular_coefficient),
            densidad_óptica: mat.optical_density
        }
    }
}

pub fn mezclar_colores(colores: &[Color]) -> Color {
    colores.iter().sum::<Color>() / colores.len() as f64
}

fn crear_color_desde_mtl(color: &mtl::Color) -> Color {
    Color::new(color.r, color.g, color.b)
}

