use nalgebra::Vector3;
use wavefront_obj::mtl;

pub type Color = Vector3<f64>;

#[derive(Clone, Copy, Debug)]
pub enum Type {
    Emitter,
    Lambertian,
    Specular
}

#[derive(Clone, Copy, Debug)]
pub struct Material {
//    nombre: String,   // no necesito nombre creo
pub tipo: Type,
    pub ambient_color: Option<Color>,      // el color base
    pub emitted_color: Option<Color>,       // si emite luz, tira este color
pub diffused_color: Option<Color>,        // para la reflexión difusa (rayos
                                            // reflejados difusos)
    pub specular_color: Option<Color>,     // para los rayos reflejados
    pub specular_coefficient: Option<f64>,   // para la reflexión especular
                                            // creo, va de 0 a 1000 parece
    pub optical_density: Option<f64> // el coeficiente de refracción
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
            optical_density: None
        }
    }
}

impl Material {
    // Todavía ni se como van a ser mis materiales, por ahora solo leo el color
    pub fn from_toml(toml: &toml::Table) -> Result<Self, anyhow::Error> {
        let error = || anyhow::anyhow!("No se pudo cargar el material");

        let color = toml.get("albedo").map(|c| c.as_array()).flatten()
            .ok_or(error())?;
        let color: Vec<f64> = color.iter().map(|c| c.as_float().ok_or(error()))
            .collect::<Result<_, _>>()?;
        anyhow::ensure!(color.len() == 3, error());

        let color = Color::new(color[0], color[1], color[2]);

        let mut material = Material::default();

        match toml.get("type").ok_or(error())?.as_str() {
            Some("mtl") => {
                // abrir archivo y eso
                todo!()
            }
            Some("Lambertian") => {
                material.tipo = Type::Lambertian;
                material.ambient_color = Some(color);
                Ok(material)
            }
            Some("Specular") => {
                material.tipo = Type::Specular;
                material.specular_color = Some(color);
                Ok(material)
            }
            Some("Emitter") => {
                material.tipo = Type::Emitter;
                material.emitted_color = Some(color);
                Ok(material)
                
            }
            Some(s) => {
                dbg!(s);
                todo!()
            }
            None => {
                todo!()
            }
        }
    }
}

impl From<&mtl::Material> for Material {
    fn from(mat: &mtl::Material) -> Self {
        Material {
            tipo: Type::Lambertian, // después ver que hacer con esto
            ambient_color: Some(create_color_from_mtl(&mat.color_ambient)),
            emitted_color: mat
                .color_emissive
                .map(|c| create_color_from_mtl(&c)),
            diffused_color: Some(create_color_from_mtl(&mat.color_diffuse)),
            specular_color: Some(create_color_from_mtl(&mat.color_specular)),
            specular_coefficient: Some(mat.specular_coefficient),
            optical_density: mat.optical_density
        }
    }
}

pub fn add_colors(c_1: &Color, c_2: &Color) -> Color {
    let c = c_1 + c_2;

    c.map(|r| r.clamp(0.0, 1.0 - 1e-10))
}
pub fn mix_colors(colores: &[Color]) -> Color {
    colores.iter().sum::<Color>() / colores.len() as f64
}

pub fn clamp_color(color: &mut Color) {
    color.x = color.x.clamp(0.0, 1.0 - 1e-10);
    color.y = color.y.clamp(0.0, 1.0 - 1e-10);
    color.z = color.z.clamp(0.0, 1.0 - 1e-10);
}

fn create_color_from_mtl(color: &mtl::Color) -> Color {
    Color::new(color.r, color.g, color.b)
}

