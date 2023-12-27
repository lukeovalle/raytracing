use crate::geometry::{Normal, Point, Ray, Vector};
use crate::shapes::Shape;

/// punto es el punto donde chocaron.
/// normal es la dirección normal del modelo en dirección saliente al objeto,
/// no la normal del mismo lado de donde venía el rayo.
/// t es el valor en el que se evaluó el rayo para el choque.
#[derive(Debug, Clone)]
pub struct Intersection {
    modelo: Shape,
    punto: Point,
    rayo_incidente: Ray,
    direction_out: Vector,
    normal: Normal,
    inside: bool, // capaz sirva esto??
    t: f64,
}

impl Intersection {
    pub fn new(
        modelo: &Shape,
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

    pub fn model(&self) -> &Shape {
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
