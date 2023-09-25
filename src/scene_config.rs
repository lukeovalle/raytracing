use crate::auxiliar;
use crate::camera::Camera;
use crate::geometry::Point;
use crate::material::{self, Material};
use crate::models::{ModelObj, Sphere, Triangle};
use crate::scene::Scene;
use crate::spectrum::{SampledSpectrum, SpectrumType};
use toml::{Table, Value};

pub fn parse_file(path: &str) -> Result<Table, anyhow::Error> {
    Ok(auxiliar::read_file(path)?.parse()?)
}

pub fn parse_camera(table: &Table) -> Result<Camera, anyhow::Error> {
    let table = table
        .get("Camera")
        .and_then(|c| c.as_table())
        .ok_or(anyhow::anyhow!("No se ha especificado la cámara."))?;
    Camera::from_toml(table)
}

impl Camera {
    fn from_toml(table: &Table) -> Result<Camera, anyhow::Error> {
        let error = || anyhow::anyhow!("No se pudo cargar la cámara.");

        let width = table
            .get("width")
            .and_then(|w| w.as_integer())
            .ok_or(error())? as u32;

        let height = table
            .get("height")
            .and_then(|h| h.as_integer())
            .ok_or(error())? as u32;

        let focal_distance = table
            .get("focal_distance")
            .and_then(|f| f.as_float())
            .ok_or(error())?;

        let field_of_view = table
            .get("field_of_view")
            .and_then(|f| f.as_float())
            .ok_or(error())?;

        let position: Vec<f64> = table
            .get("position")
            .and_then(|p| p.as_array())
            .ok_or(error())?
            .iter()
            .map(|v| v.as_float().ok_or(error()))
            .collect::<Result<_, _>>()?;
        anyhow::ensure!(position.len() == 3, error());

        let rotation: Vec<f64> = table
            .get("rotation")
            .and_then(|r| r.as_array())
            .ok_or(error())?
            .iter()
            .map(|v| v.as_float().ok_or(error()))
            .collect::<Result<_, _>>()?;
        anyhow::ensure!(rotation.len() == 3, error());

        Ok(Camera::new(
            &Point::new(position[0], position[1], position[2]),
            focal_distance,
            field_of_view,
            (rotation[0], rotation[1], rotation[2]),
            (width, height),
        ))
    }
}

pub fn parse_scene(table: &Table) -> Result<Scene, anyhow::Error> {
    let table = table
        .get("Scene")
        .and_then(|s| s.as_array())
        .ok_or(anyhow::anyhow!("No se ha especificado la escena."))?;
    Scene::from_toml(table)
}

impl Scene {
    pub fn from_toml(models: &Vec<Value>) -> Result<Scene, anyhow::Error> {
        let error = || anyhow::anyhow!("Error con la escena definida.");

        let mut scene = Scene::new();

        for model in models {
            let model = model.as_table().ok_or(error())?;
            match model.get("type").ok_or(error())?.as_str() {
                Some("Sphere") => {
                    let objeto = Sphere::from_toml(model)?;
                    scene.add_shape(&objeto.into())?;
                }
                Some("Triangle") => {
                    let objeto = Triangle::from_toml(model)?;
                    scene.add_shape(&objeto.into())?;
                }
                Some("ModelObj") => {
                    let objeto = ModelObj::from_toml(model)?;
                    scene.add_shape(&objeto.into())?;
                }
                Some(s) => {
                    dbg!(s);
                }
                None => {
                    dbg!("No type");
                }
            }
        }

        Ok(scene)
    }
}

impl ModelObj {
    pub fn from_toml(toml: &Table) -> Result<ModelObj, anyhow::Error> {
        let error = || anyhow::anyhow!("No se pudo cargar el modelo de objeto");
        let file = toml
            .get("path")
            .and_then(|p| p.as_str())
            .ok_or(error())?;

        // ver si también hay un material, agregar un parámetro a este método
        ModelObj::new(file)
    }
}

impl Sphere {
    pub fn from_toml(toml: &Table) -> Result<Sphere, anyhow::Error> {
        let error = || anyhow::anyhow!("No se pudo cargar el modelo de esfera");
        let centro: Vec<f64> = toml
            .get("center")
            .ok_or(error())?
            .as_array()
            .ok_or(error())?
            .iter()
            .map(|v| v.as_float().ok_or(error()))
            .collect::<Result<_, _>>()?;
        if centro.len() != 3 {
            return Err(error());
        }

        let centro = Point::new(centro[0], centro[1], centro[2]);
        let radio = toml
            .get("radius")
            .ok_or(error())?
            .as_float()
            .ok_or(error())?;
        let material = match toml.get("material") {
            Some(Value::Table(toml)) => Material::from_toml(toml)?,
            Some(_) => return Err(error()),
            None => Default::default(),
        };

        Ok(Sphere::new(&centro, radio, &material))
    }
}

impl Triangle {
    pub fn from_toml(toml: &Table) -> Result<Triangle, anyhow::Error> {
        let error =
            || anyhow::anyhow!("No se pudo cargar el modelo de triángulo.");
        let vértices = toml
            .get("vertices")
            .and_then(|v| v.as_array())
            .ok_or(error())?;
        anyhow::ensure!(vértices.len() == 3, error());

        let p_1 = create_point_from_toml(&vértices[0])?;
        let p_2 = create_point_from_toml(&vértices[1])?;
        let p_3 = create_point_from_toml(&vértices[2])?;

        let material = match toml.get("material") {
            Some(Value::Table(toml)) => Material::from_toml(toml)?,
            Some(_) => return Err(error()),
            None => Default::default(),
        };

        Ok(Triangle::new(&p_1, &p_2, &p_3, &material))
    }
}

impl Material {
    // Todavía ni se como van a ser mis materiales, por ahora solo leo el color
    pub fn from_toml(toml: &Table) -> Result<Self, anyhow::Error> {
        let error = || anyhow::anyhow!("No se pudo cargar el material");

        let color = toml
            .get("albedo")
            .and_then(|c| c.as_array())
            .ok_or(error())?;
        let color: Vec<f64> = color
            .iter()
            .map(|c| c.as_float().ok_or(error()))
            .collect::<Result<_, _>>()?;
        anyhow::ensure!(color.len() == 3, error());

        let type_ = toml
            .get("type")
            .ok_or(error())?
            .as_str();

        let color = SampledSpectrum::from_RGB(
            (color[0] as f32, color[1] as f32, color[2] as f32),
            match type_ {
                Some("Emitter") => SpectrumType::Illuminant,
                _ => SpectrumType::Reflectance,
            },

        );

        let mut material = Material::default();

        match type_ {
            Some("mtl") => {
                // abrir archivo y eso
                todo!()
            }
            Some("Lambertian") => {
                material.tipo = material::Type::Lambertian;
                material.ambient_color = Some(color);
                Ok(material)
            }
            Some("Specular") => {
                material.tipo = material::Type::Specular;
                material.specular_color = Some(color);
                Ok(material)
            }
            Some("Emitter") => {
                material.tipo = material::Type::Emitter;
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

pub fn create_point_from_toml(arr: &Value) -> Result<Point, anyhow::Error> {
    let error = || anyhow::anyhow!("No se pudo cargar el punto");
    let arr = arr.as_array().ok_or(error())?;
    anyhow::ensure!(arr.len() == 3, error());

    let x = arr[0].as_float().ok_or(error())?;
    let y = arr[1].as_float().ok_or(error())?;
    let z = arr[2].as_float().ok_or(error())?;

    Ok(Point::new(x, y, z))
}
