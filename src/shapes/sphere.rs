use crate::geometry::{AABB, Normal, Point, Ray};
use crate::material::Material;
use crate::shapes::common::Intersection;
use crate::shapes::model::{Model, ModelMethods};

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