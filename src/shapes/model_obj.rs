use wavefront_obj::{mtl, obj};
use crate::auxiliar::read_file;
use crate::geometry::{AABB, create_point_from_vertex, Ray};
use crate::material::Material;
use crate::shapes::{Intersection, ShapeOperations, Triangle};

#[derive(Clone)]
pub struct ModelObj {
    triángulos: Vec<Triangle>,
    material: Material,
    caja: AABB,
}

impl ModelObj {
    pub fn new(archivo: &str) -> Result<ModelObj, anyhow::Error> {
        let datos = read_file(archivo)?;
        let objetos = obj::parse(datos)?;

        let material = match objetos.material_library {
            Some(nombre) => {
                let datos = read_file(&nombre)?;
                Material::from(
                    mtl::parse(datos)?.materials.first().ok_or_else(|| {
                        anyhow::anyhow!(
                            "No se pudo cargar el material de {:?}",
                            nombre
                        )
                    })?,
                )
            }
            None => Default::default(),
        };

        let mut triángulos = Vec::new();

        for objeto in &objetos.objects {
            for geometría in &objeto.geometry {
                // Conjunto de shapes según el crate este
                for figura in &geometría.shapes {
                    if let obj::Primitive::Triangle(vtn_1, vtn_2, vtn_3) =
                        figura.primitive
                    {
                        // vtn: vértice, textura, normal
                        triángulos.push(Triangle::new(
                            &create_point_from_vertex(
                                &objeto.vertices[vtn_1.0],
                            ),
                            &create_point_from_vertex(
                                &objeto.vertices[vtn_2.0],
                            ),
                            &create_point_from_vertex(
                                &objeto.vertices[vtn_3.0],
                            ),
                            &material,
                        ));
                    }
                }
            }
        }

        let mut caja = AABB::empty();

        for triángulo in &triángulos {
            caja.resize_box(triángulo.bounding_box());
        }

        Ok(ModelObj {
            triángulos,
            material,
            caja,
        })
    }
}

impl ShapeOperations for ModelObj {
    fn intersect(&self, rayo: &Ray) -> Option<Intersection> {
        self.caja.intersect_ray(rayo)?;

        for triángulo in &self.triángulos {
            if let Some(choque) = triángulo.intersect(rayo) {
                return Some(choque);
            }
        }

        None
    }

    fn bounding_box(&self) -> &AABB {
        &self.caja
    }

    fn material(&self) -> &Material {
        &self.material
    }
}
