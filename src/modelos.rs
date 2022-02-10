use crate::geometria::{Color, Punto, Rayo, Triángulo};
use wavefront_obj::obj;
use obj::{Object, Primitive::Triangle};

pub trait Modelo {
    fn color_del_rayo(&self, rayo: &Rayo) -> Option<Color>;

    /// Devuelve el valor t en el que hay que evaluar el rayo para el choque, si es que chocan
    fn chocan(&self, rayo: &Rayo) -> Option<f64>;
}


pub struct ModeloObj{
    objetos: Vec<Object>
}

impl ModeloObj {
    pub fn new(archivo: &String) -> Result<ModeloObj, anyhow::Error> {
        Ok(ModeloObj { objetos: obj::parse(&archivo)?.objects })
    }
}

impl Modelo for ModeloObj {
    fn chocan(&self, _rayo: &Rayo) -> Option<f64> {
        for objeto in &self.objetos {
            for geometría in &objeto.geometry { // ni se como llamar a estos cosos
                for figura in &geometría.shapes {
                    match figura.primitive {
                        Triangle(vtn_1, vtn_2, vtn_3) => { // vtn: vértice, textura, normal 
                            let v_1 = objeto.vertices[vtn_1.0];
                            let v_2 = objeto.vertices[vtn_2.0];
                            let v_3 = objeto.vertices[vtn_3.0];
                        }
                        _ => {}
                    }
                }
            }
        }

       None 
    }

    fn color_del_rayo(&self, rayo: &Rayo) -> Option<Color> {
        let normalizado = rayo.dirección().normalize();
        let gris = (normalizado.x + normalizado.y + normalizado.z) / 3.0;
        Some(Color::new(gris, gris, gris))
    }
}

pub struct Esfera {
    centro: Punto,
    radio: f64
}

impl Esfera {
    pub fn new(centro: &Punto, radio: f64) -> Esfera {
        Esfera {
            centro: centro.clone(),
            radio
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

    fn color_del_rayo(&self, rayo: &Rayo) -> Option<Color> {
        if let Some(_t) = self.chocan(rayo) {
            return Some(Color::new(0.8, 0.2, 0.2));
        }

        None
    }
}

impl Modelo for Triángulo {
    fn chocan(&self, rayo: &Rayo) -> Option<f64> {
        match self.intersecar_rayo(rayo) {
            Some ((t, ..)) => {
                Some(t)
            }
            None => {
                None
            }
        }
    }

    fn color_del_rayo(&self, rayo: &Rayo) -> Option<Color> {
        if let Some(_t) = self.chocan(rayo) {
            return Some(Color::new(0.2, 0.2, 0.8));
        }

        None
    }
}

