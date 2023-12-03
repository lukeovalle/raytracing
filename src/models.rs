use crate::auxiliar::*;
use crate::geometry::{
    create_point_from_vertex, intersect_ray_and_triangle, AABB, Point,
    Ray, Vector, Normal,
};
use crate::material::Material;
use enum_dispatch::enum_dispatch;
use wavefront_obj::mtl;
use wavefront_obj::obj;

#[enum_dispatch]
pub trait ModelMethods {
    fn material(&self) -> &Material;

    /// Devuelve el valor t en el que hay que evaluar el rayo para el choque,
    /// si es que chocan
    fn intersect(&self, rayo: &Ray) -> Option<Intersection>;

    fn bounding_box(&self) -> &AABB;
}

#[allow(clippy::upper_case_acronyms)]
#[enum_dispatch(ModelMethods)]
#[derive(Clone)]
pub enum Model {
    BoxAABB(BoxAABB),
    Sphere(Sphere),
    Triangle(Triangle),
    ModelObj(ModelObj),
}

/// punto es el punto donde chocaron.
/// normal es la dirección normal del modelo en dirección saliente al objeto,
/// no la normal del mismo lado de donde venía el rayo.
/// t es el valor en el que se evaluó el rayo para el choque.
pub struct Intersection {
    modelo: Model,
    punto: Point,
    rayo_incidente: Ray,
    direction_out: Vector,
    normal: Normal,
    inside: bool, // capaz sirva esto??
    t: f64,
}

impl Intersection {
    pub fn new(
        modelo: &Model,
        punto: &Point,
        rayo: &Ray,
        normal: &Normal,
        t: f64,
    ) -> Intersection {
        Intersection {
            modelo: modelo.clone(),
            punto: *punto,
            rayo_incidente: *rayo,
            direction_out: -rayo.dir(),
            normal: *normal,
            inside: normal.dot(rayo.dir()) > 0.0,
            t,
        }
    }

    pub fn model(&self) -> &Model {
        &self.modelo
    }

    pub fn point(&self) -> &Point {
        &self.punto
    }

    pub fn incident_ray(&self) -> &Ray {
        &self.rayo_incidente
    }

    pub fn direction_out(&self) -> &Vector {
        &self.direction_out
    }
    pub fn normal(&self) -> &Normal {
        &self.normal
    }

    pub fn t(&self) -> f64 {
        self.t
    }

    pub fn invert_normal(&mut self) {
        self.normal = -self.normal;
    }
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Clone)]
pub struct BoxAABB {
    objetos: Vec<Model>,
    mat: Material, // No lo uso, está para devolver algo
    caja: AABB,
}

impl BoxAABB {
    pub fn new() -> BoxAABB {
        BoxAABB {
            objetos: Vec::new(),
            mat: Default::default(),
            caja: AABB::empty(),
        }
    }

    pub fn add_model(&mut self, modelo: &Model) {
        self.caja.resize_box(modelo.bounding_box());
        self.objetos.push(modelo.clone());
    }

    fn intersection_ray_box(&self, rayo: &Ray) -> bool {
        self.caja.intersect_ray(rayo).is_some()
    }
}

impl ModelMethods for BoxAABB {
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

    fn bounding_box(&self) -> &AABB {
        &self.caja
    }
}

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

impl ModelMethods for ModelObj {
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

#[derive(Clone, Copy)]
pub struct Sphere {
    centro: Point,
    radio: f64,
    material: Material,
    caja: AABB,
}

impl Sphere {
    pub fn new(centro: &Point, radio: f64, material: &Material) -> Sphere {
        let min =
            Point::new(centro.x - radio, centro.y - radio, centro.z - radio);
        let max =
            Point::new(centro.x + radio, centro.y + radio, centro.z + radio);

        Sphere {
            centro: *centro,
            radio,
            material: *material,
            caja: AABB::new(&min, &max),
        }
    }

    fn normal(&self, punto: &Point) -> Normal {
        (punto - self.centro).normalize()
    }
}

impl ModelMethods for Sphere {
    fn intersect(&self, rayo: &Ray) -> Option<Intersection> {
        // C centro de la esfera, r radio, P+X.t rayo. busco t de intersección
        // (P + t.X - C) * (P + t.X - C) - r² = 0
        // términos cuadráticos: a = X*X, b = 2.X.(P-C), c = (P-C)*(P-C)-r²
        // reemplazando b por 2.h, la ecuación queda (-h+-sqrt(h²-a.c))/a
        // simplifico: a = norma²(X); h = X.(P-C); c = norma²(P-C)-r²
        // X ya viene normalizado de crear el rayo, así que a = 1 siempre

        let h = rayo.dir().dot(&(rayo.origin() - self.centro));
        let c = (rayo.origin() - self.centro).norm_squared()
            - self.radio * self.radio;

        let discriminante = h * h - c;

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

        let t = if t_1 < 0.0 {
            // only t_2 >= 0
            t_2
        } else {
            // both are >= 0 and t_1 is smaller
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

        let punto =  match rayo.at(t) {
            Some(p) => p,
            None => return None,
        };

        let model = Model::from(*self);
        Some(Intersection::new(
            &model,
            &punto,
            rayo,
            &self.normal(&punto),
            t,
        ))
    }

    fn material(&self) -> &Material {
        &self.material
    }

    fn bounding_box(&self) -> &AABB {
        &self.caja
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Triangle {
    vértices: [Point; 3],
    material: Material,
    caja: AABB,
    normal: Normal,
}

impl Triangle {
    pub fn new(
        p_1: &Point,
        p_2: &Point,
        p_3: &Point,
        material: &Material,
    ) -> Triangle {
        Triangle {
            vértices: [*p_1, *p_2, *p_3],
            material: *material,
            caja: Triangle::get_box(p_1, p_2, p_3),
            normal: (p_2 - p_1).cross(&(p_3 - p_1)).normalize(),
        }
    }

    fn get_box(p_1: &Point, p_2: &Point, p_3: &Point) -> AABB {
        let min = Point::new(
            smaller_of_three(p_1.x, p_2.x, p_3.x),
            smaller_of_three(p_1.y, p_2.y, p_3.y),
            smaller_of_three(p_1.z, p_2.z, p_3.z),
        );
        let max = Point::new(
            bigger_of_three(p_1.x, p_2.x, p_3.x),
            bigger_of_three(p_1.y, p_2.y, p_3.y),
            bigger_of_three(p_1.z, p_2.z, p_3.z),
        );

        AABB::new(&min, &max)
    }

    pub fn vértice(&self, i: usize) -> Point {
        self.vértices[i]
    }

    fn normal(&self, _punto: &Point) -> Normal {
        self.normal
    }
}

impl ModelMethods for Triangle {
    fn intersect(&self, rayo: &Ray) -> Option<Intersection> {
        match intersect_ray_and_triangle(&self.vértices, rayo) {
            Some((t, ..)) => {
                let punto = match rayo.at(t) {
                    Some(p) => p,
                    None => return None,
                };
                let model = Model::from(*self);
                Some(Intersection::new(
                    &model,
                    &punto,
                    rayo,
                    &self.normal(&punto),
                    t,
                ))
            }
            None => None,
        }
    }

    fn material(&self) -> &Material {
        &self.material
    }

    fn bounding_box(&self) -> &AABB {
        &self.caja
    }
}
