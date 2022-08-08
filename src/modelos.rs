use crate::auxiliares::*;
use crate::geometria::{Caja, Punto, Rayo, crear_punto_desde_vertex};
use crate::material::{Material};
use nalgebra::Vector3;
use wavefront_obj::obj;
use wavefront_obj::mtl;

pub trait Modelo {
    fn material(&self) -> &Material;

    /// Devuelve el valor t en el que hay que evaluar el rayo para el choque, si es que chocan
    fn chocan(&self, rayo: &Rayo) -> Option<Choque>;

    fn caja_envolvente(&self) -> &Caja;
}

/// punto es el punto donde chocaron. normal es la dirección normal del modelo en dirección
/// saliente al objeto, no la normal del mismo lado de donde venía el rayo. t es el valor en el que
/// se evaluó el rayo para el choque.
pub struct Choque<'a> {
    modelo: &'a dyn Modelo,
    punto: Punto,
    rayo_incidente: Rayo,
    normal: Vector3<f64>,
    t: f64
}

impl<'a> Choque<'a> {
    pub fn new(
        modelo: &'a dyn Modelo,
        punto: &Punto,
        rayo: &Rayo,
        normal: &Vector3<f64>,
        t: f64
    ) -> Choque<'a> {
        Choque {
            modelo: modelo,
            punto: punto.clone(),
            rayo_incidente: rayo.clone(),
            normal: normal.clone(),
            t: t
        }
    }

    pub fn modelo(&self) -> &'a dyn Modelo {
        self.modelo
    }

    pub fn punto(&self) -> &Punto {
        &self.punto
    }

    pub fn rayo_incidente(&self) -> &Rayo {
        &self.rayo_incidente
    }

    pub fn normal(&self) -> &Vector3<f64> {
        &self.normal
    }

    pub fn t(&self) -> f64 {
        self.t
    }

    pub fn invertir_normal(&mut self) {
        self.normal = -self.normal;
    }
}

pub struct CajaEnvolvente {
    objetos: Vec<Box<dyn Modelo>>,
    mat: Material, // No lo uso, está para devolver algo
    caja: Caja
}

impl CajaEnvolvente {
    pub fn new() -> CajaEnvolvente {
        CajaEnvolvente {
            objetos: Vec::new(),
            mat: Default::default(),
            caja: Caja::vacía()
        }
    }

    pub fn añadir_modelo(&mut self, modelo: Box<dyn Modelo>) {
        self.caja.ampliar_caja(modelo.caja_envolvente());
        self.objetos.push(modelo);
    }

    fn intersección_rayo_caja(&self, rayo: &Rayo) -> bool {
        self.caja.intersección(rayo).is_some()
    }
}

impl Modelo for CajaEnvolvente {
    fn material(&self) -> &Material {
        &self.mat
    }

    fn chocan(&self, rayo: &Rayo) -> Option<Choque> {
        if !self.intersección_rayo_caja(rayo) {
            return None;
        }

        for obj in &self.objetos {
            let choque = obj.chocan(rayo);
            if choque.is_some() {
                return choque;
            }
        }
        None
    }

    fn caja_envolvente(&self) -> &Caja {
        &self.caja
    }
}


pub struct ModeloObj {
    triángulos: Vec<Triángulo>,
    material: Material,
    caja: Caja
}

impl ModeloObj {
    pub fn new(archivo: &str) -> Result<ModeloObj, anyhow::Error> {
        let datos = leer_archivo(archivo)?;
        let objetos = obj::parse(datos)?;

        let material = match objetos.material_library {
            Some(nombre) => {
                let datos = leer_archivo(&nombre)?;
                Material::from(mtl::parse(datos)?.materials.first()
                    .ok_or(anyhow::anyhow!("No se pudo cargar el material de {:?}", nombre))?)
            }
            None => Default::default()
        };

        let mut triángulos = Vec::new();

        for objeto in &objetos.objects {
            for geometría in &objeto.geometry { // Conjunto de shapes según el crate este
                for figura in &geometría.shapes {
                    match figura.primitive {
                        obj::Primitive::Triangle(vtn_1, vtn_2, vtn_3) => { // vtn: vértice, textura, normal
                            triángulos.push(Triángulo::new(
                                    &crear_punto_desde_vertex(&objeto.vertices[vtn_1.0]),
                                    &crear_punto_desde_vertex(&objeto.vertices[vtn_2.0]),
                                    &crear_punto_desde_vertex(&objeto.vertices[vtn_3.0]),
                                    &material
                            ));
                        }
                        _ => {}
                    }
                }
            }
        }

        let mut caja = Caja::vacía();

        for triángulo in &triángulos {
            caja.ampliar_caja(triángulo.caja_envolvente());
        }


        Ok(ModeloObj {
            triángulos: triángulos,
            material: material,
            caja: caja
        })
    }
}

impl Modelo for ModeloObj {
    fn chocan(&self, rayo: &Rayo) -> Option<Choque> {
        if self.caja.intersección(rayo).is_none() {
            return None;
        }

        for triángulo in &self.triángulos {
            match triángulo.chocan(rayo) {
                Some(choque) => { return Some(choque); }
                None => {}
            }
        }

        None 
    }

    fn caja_envolvente(&self) -> &Caja {
        &self.caja
    }

    fn material(&self) -> &Material {
        &self.material
    }
}

pub struct Esfera {
    centro: Punto,
    radio: f64,
    material: Material,
    caja: Caja
}

impl Esfera {
    pub fn new(centro: &Punto, radio: f64, material: &Material) -> Esfera {
        let min = Punto::new(centro.x - radio, centro.y - radio, centro.z - radio);
        let max = Punto::new(centro.x + radio, centro.y + radio, centro.z + radio);

        Esfera {
            centro: centro.clone(),
            radio,
            material: material.clone(),
            caja: Caja::new(&min, &max)
        }
    }

    fn normal(&self, punto: &Punto) -> Vector3<f64> {
        (punto - self.centro).normalize()
    }
}

impl Modelo for Esfera {
    fn chocan(&self, rayo: &Rayo) -> Option<Choque> {
        // C centro de la esfera, r radio, P+X.t rayo. busco t de intersección
        // (P + t.X - C) * (P + t.X - C) - r² = 0
        // términos cuadráticos: a = X*X, b = 2.X.(P-C), c = (P-C)*(P-C)-r²
        // reemplazando b por 2.h, la ecuación queda (-h+-sqrt(h²-a.c))/a
        // simplifico: a = norma²(X); h = X.(P-C); c = norma²(P-C)-r²
        let a = rayo.dirección().norm_squared();
        let h = rayo.dirección().dot(&(rayo.origen() - self.centro));
        let c = (rayo.origen() - self.centro).norm_squared() - self.radio*self.radio;

        let discriminante = h*h - a*c;

        if discriminante < 0.0 {
            return None;
        }

        let t_1 = (-h - discriminante.sqrt()) / a;
        let t_2 = (-h + discriminante.sqrt()) / a;

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

        let punto = rayo.evaluar(t);

        Some( Choque::new(self, &punto, rayo, &self.normal(&punto), t) )
    }

    fn material(&self) -> &Material {
        &self.material
    }

    fn caja_envolvente(&self) -> &Caja {
        &self.caja
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Triángulo {
    vértices: [Punto; 3],
    material: Material,
    caja: Caja
}

impl Triángulo {
    pub fn new(p_1: &Punto, p_2: &Punto, p_3: &Punto, material: &Material) -> Triángulo {
        Triángulo {
            vértices: [p_1.clone(), p_2.clone(), p_3.clone()],
            material: material.clone(),
            caja: Triángulo::calcular_caja(p_1, p_2, p_3)
        }
    }

    fn calcular_caja(p_1: &Punto, p_2: &Punto, p_3: &Punto) -> Caja {
        let min = Punto::new(
            menor_de_tres(p_1.x, p_2.x, p_3.x),
            menor_de_tres(p_1.y, p_2.y, p_3.y),
            menor_de_tres(p_1.z, p_2.z, p_3.z)
        );
        let max = Punto::new(
            mayor_de_tres(p_1.x, p_2.x, p_3.x),
            mayor_de_tres(p_1.y, p_2.y, p_3.y),
            mayor_de_tres(p_1.z, p_2.z, p_3.z),
        );

        Caja::new(&min, &max)
    }


    pub fn vértice(&self, i: usize) -> Punto {
        self.vértices[i]
    }

    fn normal(&self, _punto: &Punto) -> Vector3<f64> {
        (self.vértice(1) - self.vértice(0)).cross(&(self.vértice(2) - self.vértice(0))).normalize()
    }
}


impl Modelo for Triángulo {
    fn chocan(&self, rayo: &Rayo) -> Option<Choque> {
        match crate::geometria::intersecar_rayo_y_triángulo(&self.vértices, rayo) {
            Some ((t, ..)) => {
                let punto = rayo.evaluar(t);
                Some( Choque::new(self, &punto, rayo, &self.normal(&punto), t) )
            }
            None => {
                None
            }
        }
    }

    fn material(&self) -> &Material {
        &self.material
    }

    fn caja_envolvente(&self) -> &Caja {
        &self.caja
    }
}

