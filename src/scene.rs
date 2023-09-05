use crate::models::{Intersection, Model, ModelMethods};
use crate::material::{Type, Color};
use crate::geometry::Ray;

#[derive(Clone)]
pub struct Scene {
    objetos: Vec<Model>
}

impl Scene {
    pub fn new() -> Scene {
        Scene {
            objetos: Vec::new()
        }
    }

    pub fn from_toml(models: &Vec<toml::value::Value>) -> Result<Scene, anyhow::Error> {
        let error = || anyhow::anyhow!("Error con la escena definida.");

        let mut scene = Scene::new();

        for model in models {
            let model = model.as_table().ok_or(error())?;
            match model.get("type").ok_or(error())?.as_str() {
                Some("Sphere") => {
                    let objeto = crate::models::Sphere::from_toml(model)?;
                    scene.add_shape(&objeto.into())?;
                }
                Some("Triangle") => {
                    let objeto = crate::models::Triangle::from_toml(model)?;
                    scene.add_shape(&objeto.into())?;
                }
                Some("ModelObj") => {
                    let objeto = crate::models::ModelObj::from_toml(model)?;
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

    pub fn add_shape(
        &mut self,
        objeto: &Model
    ) -> Result<(), anyhow::Error> {
        self.objetos.push(objeto.clone());
        Ok(())
    }

    fn trace_ray(&self, rayo: &Ray, iteraciones: usize ) -> Color {
        if iteraciones == 0 {
            return Color::zeros();
        }

        match self.intersect_ray(rayo) {
            Some(mut choque) => {
                if choque.normal().dot(rayo.direction()) > 0.0 {
                    // la normal apunta para "adentro" del objeto
                    choque.invert_normal()
                } else {
                    // apunta para afuera
                    // no hago nada por ahora
                }

                // devuelvo el color en el punto
                self.shade_point(&choque, iteraciones)
            }
            None => { Color::zeros() }
        }
    }

    pub fn shade_point(
        &self,
        choque: &Intersection,
        iteraciones: usize
    ) -> Color {
        let objeto = choque.model();
        let punto = choque.point();
        let incidente = choque.incident_ray().direction();
        let normal = choque.normal();

        match objeto.material().tipo {
            Type::Emitter => {
                if let Some(col) = objeto.material().emitted_color{
                    col
                } else {
                    Color::zeros()
                }
            }
            Type::Lambertian => {
                let dirección = crate::geometry::random_versor_cos_density(
                    normal
                );
                let rayo = Ray::new(&(punto + normal * 1e-10), &dirección);

                if let Some(col) = objeto.material().ambient_color{
                    //sumar_colores(&self.trazar_rayo(&rayo, iteraciones - 1), &col)
                    self.trace_ray(&rayo, iteraciones - 1).component_mul(&col)
                } else {
                    Color::zeros()
                }
            }
            Type::Specular => {
                let color = if let Some(col) = objeto.material()
                    .specular_color {
                    col
                } else {
                    Color::new(1.0, 1.0, 1.0)
                };

                // si i es el rayo incidente, n es la normal, y r el reflejado respecto a esa
                // normal, entonces r = i + 2.a, 2.a es la diferencia entre ambos vectores.
                // a tiene dirección de n y módulo i*cos(angulo(i,n)). o sea a = <d, n>.n
                // Asumo que n viene normalizado
                let dirección = incidente - normal * (2.0 * incidente.dot(normal));

                let rayo = Ray::new(&(punto + normal * 1e-10), &dirección);

                let tita = normal.dot(&dirección);
                // Aproximación de Schlick a las ecuaciones de Fresnel
                // R(t) = R_0 + (1 - R_0)*(1 - cos(t))⁵
                let color = color.map(|r| r + (1.0 - r) * (1.0 - tita.cos()).powi(5));

                //sumar_colores(&self.trazar_rayo(&rayo, iteraciones - 1), &color)
                self.trace_ray(&rayo, iteraciones - 1).component_mul(&color)
            }
        }

        // por ahora saco los shadow rays
        /*
        for luz in &self.luces {
            let dirección = luz.fuente() - punto;
            // corro el origen del rayo para que no choque con el objeto que quiero sombrear
            let rayo = Rayo::new(&(punto + normal * 1e-10), &dirección);
            let obstáculo = self.intersecar_rayo(&rayo);

            if obstáculo.is_none() {
                if let Some(col) = objeto.material().color_ambiente {
                    colores.push(col * luz.atenuación(punto));
                }
            }
        }
        */
    }

    // Si el rayo choca contra algo, devuelve el coso chocado y el t a evaluar en el rayo para el
    // choque.
    pub fn intersect_ray(&self, rayo: &Ray) -> Option<Intersection> {
        // el objeto más cercano que atraviesa el rayo
        let menor = self.objetos.iter()
            .filter_map(|obj| obj.intersect(rayo) )
            .reduce(|menor, actual| {
                if actual.t() < menor.t() {
                    actual
                } else {
                    menor
                }
            });
        
        menor
    }
}

