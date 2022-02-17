use crate::geometria::{Punto, Rayo};
use crate::material::{Material};
use nalgebra::Vector3;
use wavefront_obj::obj;
use obj::{Object, Primitive::Triangle};

pub trait Modelo {
    fn material(&self) -> &Material;

    /// Devuelve el valor t en el que hay que evaluar el rayo para el choque, si es que chocan
    fn chocan(&self, rayo: &Rayo) -> Option<f64>;

    fn normal(&self, punto: &Punto) -> Vector3<f64>;
}


pub struct ModeloObj {
    objetos: Vec<Object>,
    material: Material
}

impl ModeloObj {
    pub fn new(archivo: &String) -> Result<ModeloObj, anyhow::Error> {
        Ok(ModeloObj {
            objetos: obj::parse(&archivo)?.objects,
            material: Default::default() // después cargar el archivo
        })
    }
}

impl Modelo for ModeloObj {
    fn chocan(&self, _rayo: &Rayo) -> Option<f64> {
        for objeto in &self.objetos {
            for geometría in &objeto.geometry { // ni se como llamar a estos cosos
                for figura in &geometría.shapes {
                    match figura.primitive {
                        Triangle(vtn_1, vtn_2, vtn_3) => { // vtn: vértice, textura, normal 
                            let _v_1 = objeto.vertices[vtn_1.0];
                            let _v_2 = objeto.vertices[vtn_2.0];
                            let _v_3 = objeto.vertices[vtn_3.0];
                        }
                        _ => {}
                    }
                }
            }
        }

       None 
    }

    fn material(&self) -> &Material {
        &self.material
    }

    fn normal(&self, _punto: &Punto) -> Vector3<f64> {
        Vector3::new(1.0, 1.0, 1.0)
    }
}

pub struct Esfera {
    centro: Punto,
    radio: f64,
    material: Material
}

impl Esfera {
    pub fn new(centro: &Punto, radio: f64, material: &Material) -> Esfera {
        Esfera {
            centro: centro.clone(),
            radio,
            material: material.clone()
        }
    }
}

impl Modelo for Esfera {
    fn chocan(&self, rayo: &Rayo) -> Option<f64> {
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
        } else if t_1 < 0.0 {
            return Some(t_2);
        } else if t_2 < 0.0 {
            return Some(t_1);
        } else if t_1 < t_2 {
            return Some(t_1);
        } else {
            return Some(t_2);
        }
    }

    fn material(&self) -> &Material {
        &self.material
    }

    fn normal(&self, punto: &Punto) -> Vector3<f64> {
        punto - self.centro
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Triángulo {
    vértices: [Punto; 3],
    material: Material
}

impl Triángulo {
    pub fn new(p_1: &Punto, p_2: &Punto, p_3: &Punto, material: &Material) -> Triángulo {
        Triángulo {
            vértices: [p_1.clone(), p_2.clone(), p_3.clone()],
            material: material.clone()
        }
    }

    pub fn vértice(&self, i: usize) -> Punto {
        self.vértices[i]
    }
}


impl Modelo for Triángulo {
    fn chocan(&self, rayo: &Rayo) -> Option<f64> {
        match crate::geometria::intersecar_rayo_y_triángulo(&self.vértices, rayo) {
            Some ((t, ..)) => {
                Some(t)
            }
            None => {
                None
            }
        }
    }

    fn material(&self) -> &Material {
        &self.material
    }

    fn normal(&self, _punto: &Punto) -> Vector3<f64> {
        (self.vértice(1) - self.vértice(0)).cross(&(self.vértice(2) - self.vértice(0)))
    }
}

