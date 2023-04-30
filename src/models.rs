use crate::auxiliar::*;
use crate::geometry::{BoundingBox, Point, Ray, create_point_from_vertex};
use crate::material::{Material};
use nalgebra::Vector3;
use wavefront_obj::obj;
use wavefront_obj::mtl;

pub trait Model {
    fn material(&self) -> &Material;

    /// Devuelve el valor t en el que hay que evaluar el rayo para el choque, si es que chocan
    fn intersect(&self, rayo: &Ray) -> Option<Intersection>;

    fn bounding_box(&self) -> &BoundingBox;
}

/// punto es el punto donde chocaron. normal es la dirección normal del modelo en dirección
/// saliente al objeto, no la normal del mismo lado de donde venía el rayo. t es el valor en el que
/// se evaluó el rayo para el choque.
pub struct Intersection<'a> {
    modelo: &'a dyn Model,
    punto: Point,
    rayo_incidente: Ray,
    normal: Vector3<f64>,
    t: f64
}

impl<'a> Intersection<'a> {
    pub fn new(
        modelo: &'a dyn Model,
        punto: &Point,
        rayo: &Ray,
        normal: &Vector3<f64>,
        t: f64
    ) -> Intersection<'a> {
        Intersection {
            modelo,
            punto: *punto,
            rayo_incidente: *rayo,
            normal: *normal,
            t
        }
    }

    pub fn model(&self) -> &'a dyn Model {
        self.modelo
    }

    pub fn point(&self) -> &Point {
        &self.punto
    }

    pub fn incident_ray(&self) -> &Ray {
        &self.rayo_incidente
    }

    pub fn normal(&self) -> &Vector3<f64> {
        &self.normal
    }

    pub fn t(&self) -> f64 {
        self.t
    }

    pub fn invert_normal(&mut self) {
        self.normal = -self.normal;
    }
}

pub struct AABB {
    objetos: Vec<Box<dyn Model>>,
    mat: Material, // No lo uso, está para devolver algo
    caja: BoundingBox
}

impl AABB {
    pub fn new() -> AABB {
        AABB {
            objetos: Vec::new(),
            mat: Default::default(),
            caja: BoundingBox::empty()
        }
    }

    pub fn add_model(&mut self, modelo: Box<dyn Model>) {
        self.caja.resize_box(modelo.bounding_box());
        self.objetos.push(modelo);
    }

    fn intersection_ray_box(&self, rayo: &Ray) -> bool {
        self.caja.intersection(rayo).is_some()
    }
}

impl Model for AABB {
    fn material(&self) -> &Material {
        &self.mat
    }

    fn intersect(&self, rayo: &Ray) -> Option<Intersection> {
        if !self.intersection_ray_box(rayo) {
            return None;
        }

        for obj in &self.objetos {
            let choque = obj.intersect(rayo);
            if choque.is_some() {
                return choque;
            }
        }
        None
    }

    fn bounding_box(&self) -> &BoundingBox {
        &self.caja
    }
}


pub struct ModelObj {
    triángulos: Vec<Triangle>,
    material: Material,
    caja: BoundingBox
}

impl ModelObj {
    pub fn new(archivo: &str) -> Result<ModelObj, anyhow::Error> {
        let datos = read_file(archivo)?;
        let objetos = obj::parse(datos)?;

        let material = match objetos.material_library {
            Some(nombre) => {
                let datos = read_file(&nombre)?;
                Material::from(mtl::parse(datos)?.materials.first()
                    .ok_or_else(|| anyhow::anyhow!("No se pudo cargar el material de {:?}", nombre))?)
            }
            None => Default::default()
        };

        let mut triángulos = Vec::new();

        for objeto in &objetos.objects {
            for geometría in &objeto.geometry { // Conjunto de shapes según el crate este
                for figura in &geometría.shapes {
                    if let obj::Primitive::Triangle(vtn_1, vtn_2, vtn_3) = figura.primitive {
                        // vtn: vértice, textura, normal
                        triángulos.push(Triangle::new(
                                &create_point_from_vertex(&objeto.vertices[vtn_1.0]),
                                &create_point_from_vertex(&objeto.vertices[vtn_2.0]),
                                &create_point_from_vertex(&objeto.vertices[vtn_3.0]),
                                &material
                        ));
                    }
                }
            }
        }

        let mut caja = BoundingBox::empty();

        for triángulo in &triángulos {
            caja.resize_box(triángulo.bounding_box());
        }


        Ok(ModelObj {
            triángulos,
            material,
            caja
        })
    }
}

impl Model for ModelObj {
    fn intersect(&self, rayo: &Ray) -> Option<Intersection> {
        self.caja.intersection(rayo)?;

        for triángulo in &self.triángulos {
            if let Some(choque) = triángulo.intersect(rayo) {
                return Some(choque);
            }
        }

        None 
    }

    fn bounding_box(&self) -> &BoundingBox {
        &self.caja
    }

    fn material(&self) -> &Material {
        &self.material
    }
}

pub struct Sphere {
    centro: Point,
    radio: f64,
    material: Material,
    caja: BoundingBox
}

impl Sphere {
    pub fn new(centro: &Point, radio: f64, material: &Material) -> Sphere {
        let min = Point::new(centro.x - radio, centro.y - radio, centro.z - radio);
        let max = Point::new(centro.x + radio, centro.y + radio, centro.z + radio);

        Sphere {
            centro: *centro,
            radio,
            material: *material,
            caja: BoundingBox::new(&min, &max)
        }
    }

    fn normal(&self, punto: &Point) -> Vector3<f64> {
        (punto - self.centro).normalize()
    }
}

impl Model for Sphere {
    fn intersect(&self, rayo: &Ray) -> Option<Intersection> {
        // C centro de la esfera, r radio, P+X.t rayo. busco t de intersección
        // (P + t.X - C) * (P + t.X - C) - r² = 0
        // términos cuadráticos: a = X*X, b = 2.X.(P-C), c = (P-C)*(P-C)-r²
        // reemplazando b por 2.h, la ecuación queda (-h+-sqrt(h²-a.c))/a
        // simplifico: a = norma²(X); h = X.(P-C); c = norma²(P-C)-r²
        // X ya viene normalizado de crear el rayo, así que a = 1 siempre

        let h = rayo.direction().dot(&(rayo.origin() - self.centro));
        let c = (rayo.origin() - self.centro).norm_squared() - self.radio*self.radio;

        let discriminante = h*h - c;

        // No intersection
        if discriminante < 0.0 {
            return None;
        }

        // t_1 is always smaller than t_2
        let t_1 = -h - discriminante.sqrt();
        let t_2 = -h + discriminante.sqrt();

        // both solutions are in the other direction
        if t_2 < 0.0 {
            return None;
        }

        let t = if t_1 < 0.0 { // only t_2 >= 0
            t_2
        } else { // both are >= 0 and t_1 is smaller
            t_1
        };

        /*
        if t_1 < 0.0 && t_2 < 0.0 {
            return None;
        }

        let t = if t_1 < 0.0 {
            t_2
        } else if t_2 < 0.0 {
            t_1
        } else if t_1 < t_2 {
            t_1
        } else {
            t_2
        };
        */

        let punto = rayo.evaluate(t);

        Some( Intersection::new(self, &punto, rayo, &self.normal(&punto), t) )
    }

    fn material(&self) -> &Material {
        &self.material
    }

    fn bounding_box(&self) -> &BoundingBox{
        &self.caja
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Triangle {
    vértices: [Point; 3],
    material: Material,
    caja: BoundingBox,
    normal: Vector3<f64>
}

impl Triangle {
    pub fn new(p_1: &Point, p_2: &Point, p_3: &Point, material: &Material) -> Triangle {
        Triangle {
            vértices: [*p_1, *p_2, *p_3],
            material: *material,
            caja: Triangle::get_box(p_1, p_2, p_3),
            normal: (p_2 - p_1).cross(&(p_3 - p_1)).normalize()
        }
    }

    fn get_box(p_1: &Point, p_2: &Point, p_3: &Point) -> BoundingBox {
        let min = Point::new(
            smaller_of_three(p_1.x, p_2.x, p_3.x),
            smaller_of_three(p_1.y, p_2.y, p_3.y),
            smaller_of_three(p_1.z, p_2.z, p_3.z)
        );
        let max = Point::new(
            bigger_of_three(p_1.x, p_2.x, p_3.x),
            bigger_of_three(p_1.y, p_2.y, p_3.y),
            bigger_of_three(p_1.z, p_2.z, p_3.z),
        );

        BoundingBox::new(&min, &max)
    }


    pub fn vértice(&self, i: usize) -> Point {
        self.vértices[i]
    }

    fn normal(&self, _punto: &Point) -> Vector3<f64> {
        self.normal
    }
}


impl Model for Triangle {
    fn intersect(&self, rayo: &Ray) -> Option<Intersection> {
        match crate::geometry::intersect_ray_and_triangle(&self.vértices, rayo) {
            Some ((t, ..)) => {
                let punto = rayo.evaluate(t);
                Some( Intersection::new(self, &punto, rayo, &self.normal(&punto), t) )
            }
            None => {
                None
            }
        }
    }

    fn material(&self) -> &Material {
        &self.material
    }

    fn bounding_box(&self) -> &BoundingBox {
        &self.caja
    }
}

