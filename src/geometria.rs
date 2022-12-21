use nalgebra::{Matrix3, Point3, Vector3};

pub type Punto = Point3<f64>;

pub fn crear_punto_desde_vertex(vertex: &wavefront_obj::obj::Vertex) -> Punto {
    Punto::new(vertex.x, vertex.y, vertex.z)
}

#[derive(Clone, Copy, Debug)]
pub struct Rayo {
    origen: Punto,
    dirección: Vector3<f64>
}

impl Rayo {
    pub fn new(origen: &Punto, dirección: &Vector3<f64>) -> Rayo {
        Rayo {
            origen: *origen,
            dirección: dirección.clone().normalize()
        }
    }

    pub fn origen(&self) -> &Punto {
        &self.origen
    }

    pub fn dirección(&self) -> &Vector3<f64> {
        &self.dirección
    }

    pub fn evaluar(&self, t: f64) -> Punto {
        self.origen + self.dirección * t
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Rectángulo(pub Punto, pub Punto, pub Punto, pub Punto);

/// Hexaedro con caras ortogonales al sistema de coordenadas canónico, de cuatro lados y ángulos
/// rectos.
/// min y max contiene los componentes (x, y, z) mínimos y máximos de cada vértice.
#[derive(Clone, Copy, Debug)]
pub struct Caja {
    min: Punto,
    max: Punto
}

impl Caja {
    pub fn new(min: &Punto, max: &Punto) -> Caja {
        Caja { min: *min, max: *max }
    }

    pub fn vacía() -> Caja {
        Caja::new(&Punto::origin(), &Punto::origin())
    }
/*
    pub fn min(&self) -> Punto {
        self.min
    }

    pub fn max(&self) -> Punto {
        self.max
    }
*/
    fn envolver_cajas(caja_1: &Caja, caja_2: &Caja) -> Caja {
        let x_min = if caja_1.min.x < caja_2.min.x { caja_1.min.x } else { caja_2.min.x };
        let x_max = if caja_1.max.x > caja_2.max.x { caja_1.max.x } else { caja_2.max.x };
        let y_min = if caja_1.min.y < caja_2.min.y { caja_1.min.y } else { caja_2.min.y };
        let y_max = if caja_1.max.y > caja_2.max.y { caja_1.max.y } else { caja_2.max.y };
        let z_min = if caja_1.min.z < caja_2.min.z { caja_1.min.z } else { caja_2.min.z };
        let z_max = if caja_1.max.z > caja_2.max.z { caja_1.max.z } else { caja_2.max.z };

        Caja { min: Punto::new(x_min, y_min, z_min), max: Punto::new(x_max, y_max, z_max) }
    }

    pub fn ampliar_caja(&mut self, otra: &Caja) {
        *self = Caja::envolver_cajas(self, otra);
    }

    pub fn intersección(&self, rayo: &Rayo) -> Option<f64> {
        let mut mínimo_intervalo = 0.0;
        let mut máximo_intervalo = f64::INFINITY;

        // Busco t_min y t_max respecto a X
        // La idea es que si alguno de estos es menor a 0, o si t_min es mayor a t_max, entonces la
        // caja no es atravesada por el rayo. La lógica va a ser la misma para los tres ejes, si
        // no puedo probar que no se chocan en cada uno de los tres ejes, entonces se chocan.
        //
        // Se tienen que cumplir seis ecuaciones de 13 incógnitas, estas dos son para x pero la
        // idea es similar en los otros dos ejes.
        // P + t.d = (x_min, 0, 0) + n.(0,1,0) + m.(0,0,1)
        // P + t.d = (x_max, 0, 0) + a.(0,1,0) + b.(0,0,1)
        // con P y d origen y dirección del rayo, x_min y x_max bordes de la caja, y (t,n,m,a,b)
        // incógnitas.
        // de acá se puede despejar con P=(p_x, p_y), d=(d_x, d_y)
        // p_x + t_0.d_x = x_min
        // p_x + t_1.d_x = x_max
        // los casos borde donde el rayo va a chocar con el borde x de la caja sin estar dentro. 
        // Considerar que si d_x = 0, entonces solo hay que analizar si x_min < p_x < x_max.
        // De otro modo, se despeja que t_0 = (x_min - p_x)/d_x y t_1 = (x_max - p_x)/d_x
        // Considerando que x_min y x_max tienen un orden conocido, para saber en que caso
        // t_min = t_0 y en que caso t_min = t_1 (y lo mismo para t_max), hay que analizar la
        // dirección d_x. si d_x > 0, t_min = t_0, si d_x < 0, t_min = t_1
        //
        // Toda esta lógica se aplica de igual manera para los ejes Y y Z

        if rayo.dirección.x == 0.0 {
            if rayo.origen.x < self.min.x || rayo.origen.x > self.max.x {
                return None;
            }
        } else {
            let t_0 = (self.min.x - rayo.origen.x) / rayo.dirección.x;
            let t_1 = (self.max.x - rayo.origen.x) / rayo.dirección.x;

            let (t_min, t_max) = if rayo.dirección.x > 0.0 {
                (t_0, t_1)
            } else {
                (t_1, t_0)
            };

            if t_min > mínimo_intervalo {
                mínimo_intervalo = t_min;
            }
            if t_max < máximo_intervalo {
                máximo_intervalo = t_max;
            }

            if mínimo_intervalo > máximo_intervalo {
                return None;
            }
        }

        // En el eje Y
        if rayo.dirección.y == 0.0 {
            if rayo.origen.y < self.min.y || rayo.origen.y > self.max.y {
                return None;
            }
        } else {
            let t_0 = (self.min.y - rayo.origen.y) / rayo.dirección.y;
            let t_1 = (self.max.y - rayo.origen.y) / rayo.dirección.y;

            let (t_min, t_max) = if rayo.dirección.y > 0.0 {
                (t_0, t_1)
            } else {
                (t_1, t_0)
            };

            if t_min > mínimo_intervalo {
                mínimo_intervalo = t_min;
            }
            if t_max < máximo_intervalo {
                máximo_intervalo = t_max;
            }
            if mínimo_intervalo > máximo_intervalo {
                return None;
            }
        }

        // En el eje Z
        if rayo.dirección.z == 0.0 {
            if rayo.origen.z < self.min.z || rayo.origen.z > self.max.z {
                return None;
            }
        } else {
            let t_0 = (self.min.z - rayo.origen.z) / rayo.dirección.z;
            let t_1 = (self.max.z - rayo.origen.z) / rayo.dirección.z;

            let (t_min, t_max) = if rayo.dirección.z > 0.0 {
                (t_0, t_1)
            } else {
                (t_1, t_0)
            };

            if t_min > mínimo_intervalo {
                mínimo_intervalo = t_min;
            }
            if t_max < máximo_intervalo {
                máximo_intervalo = t_max;
            }
            if mínimo_intervalo > máximo_intervalo {
                return None;
            }
        }

        Some(mínimo_intervalo)
    }
}

// Möller–Trumbore intersection algorithm
#[allow(non_snake_case)]
pub fn intersecar_rayo_y_triángulo(vértices: &[Punto], rayo: &Rayo) -> Option<(f64, f64, f64)>{
    // Buscá el algoritmo ese en wikipedia, hay un pdf.
    // sean:
    // D: dirección normalizada del rayo
    // O: origen del rayo
    // V_0, V_1, V_2: vectores del triángulo
    // E_1 = V_1 - V_0
    // E_2 = V_2 - V_0
    // T = O - V_0
    // Resolver el sistema de ecuaciones [-D, E_1, E_2].[t, u, v]^t = T
    // [ -D.x | E_1.x | E_2.x ]   [ t ]   [ T.x ]
    // [ -D.y | E_1.y | E_2.y ] . [ u ] = [ T.y ]
    // [ -D.z | E_1.z | E_2.z ]   [ v ]   [ T.z ]
    //
    // Aplicando la regla de Cramer,
    // [ t ]          1           [ |  T, E_1, E_2 | ]
    // [ u ] = ---------------- . [ | -D,  T , E_2 | ]  (lo de la derecha es un vector)
    // [ v ]   | -D, E_1, E_2 |   [ | -D, E_1,  T  | ]
    //
    // propiedad de determinantes: |u, v, w| = -(u x w).v = -(w x v).u
    // Reemplazo con:
    // P = D x E_2
    // Q = T x E_1
    // [ t ]     1     [ Q.E_2 ]
    // [ u ] = ----- . [  P.T  ]
    // [ v ]   P.E_1   [  Q.D  ]

    // Preparo E_1, E_2, T, D
    let E_1 = vértices[1] - vértices[0];
    let E_2 = vértices[2] - vértices[0];
    let T = rayo.origen() - vértices[0];
    let D = rayo.dirección();

    // creo P, Q
    let P = D.cross(&E_2);
    let Q = T.cross(&E_1);

    // busco determinante
    let det = P.dot(&E_1);
    if det.abs() < 1e-10 {
        return None;
    }
    let det_inverso = det.recip();

    let t = det_inverso * Q.dot(&E_2);
    if t < 0.0 {
        return None;
    }

    let u = det_inverso * P.dot(&T);
    if !(0.0..=1.0).contains(&u) {
//    if u < 0.0 || u > 1.0 {
        return None;
    }

    let v = det_inverso * Q.dot(D);
    if v < 0.0 || u + v > 1.0 {
        return None;
    }

    Some((t, u, v))
    
    /*
    // ya fue todo resuelvo con matrices
    let matriz = nalgebra::Matrix3::from_columns(&[
        rayo.dirección() * -1.0,
        vértices[1] - vértices[0],
        vértices[2] - vértices[0]
    ]);

    let b = rayo.origen() - vértices[0];

    let descomposición = matriz.qr();
    let solución = descomposición.solve(&b);

    match solución {
        Some(sol) => {
            let (t, u, v) = (sol[0], sol[1], sol[2]);

            if t < 0.0 || u < 0.0 || v < 0.0 || u + v > 1.0 {
                return None;
            }

            Some((t, u, v))
        }
        None  => { None }
    }
    */
}

/// Devuelve un versor aleatorio con función densidad p(t) = (1/pi)*cos(t), con t el ángulo entre
/// el versor generado y la normal pasada como parámetro. o sea es más probable que el versor esté
/// cerca de la normal
pub fn versor_aleatorio_densidad_cos(normal: &Vector3<f64>) -> Vector3<f64> {
    let sen_tita = rand::random::<f64>().sqrt();        // sen(θ) = sqrt(R_1)
    let cos_tita = (1.0 - sen_tita*sen_tita).sqrt();    // cos(θ) = sqrt( 1 - sen(θ)² )
    let phi: f64 = 2.0 * std::f64::consts::PI * rand::random::<f64>();  // φ = 2.π.R_2

    let v: Vector3<f64> = Vector3::new(phi.cos() * sen_tita, phi.sin() * sen_tita, cos_tita);

    crear_base_usando_normal(normal) * v
}

/// Devuelve una matriz de cambio de base a la canónica, siendo la base original una creada tomando
/// el versor k, y dos versores cualquiera que sean ortogonales a k
fn crear_base_usando_normal(normal: &Vector3<f64>) -> Matrix3<f64> {
    let mut b_1: Vector3<f64>; 

    // si la normal está cerca del eje X uso el eje Y, si no uso el X
    if normal.x.abs() > 0.9 {
        b_1 = Vector3::new(0.0, 1.0, 0.0);
    } else {
        b_1 = Vector3::new(1.0, 0.0, 0.0);
    }

    b_1 -= normal * b_1.dot(normal);   // b_1 ortogonal a normal
    b_1 *= 1.0/b_1.norm();              // b_1 normalizado

    let b_2 = normal.cross(&b_1);

    Matrix3::from_columns(&[b_1, b_2, *normal])
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evaluar_rayo() {
        let rayo = Rayo::new(&Punto::new(1.0, 1.0, 2.0), &Vector3::new(2.0, 2.0, 1.0));

        assert!((rayo.evaluar(3.0) - Punto::new(3.0, 3.0, 3.0)).norm().abs() < f64::EPSILON);
    }

    #[test]
    fn triángulo_interseca_rayo() {
        let vértices = [
            Punto::new(1.0, -1.0, 0.0),
            Punto::new(1.0,  1.0, 0.0),
            Punto::new(1.0,  0.0, 1.0)
        ];
        let rayo = Rayo::new(&Punto::new(0.0, 0.0, 0.0), &Vector3::new(1.0, 0.0, 0.5));

        assert!(intersecar_rayo_y_triángulo(&vértices, &rayo).is_some());
    }

    #[test]
    fn triángulo_no_interseca_rayo() {
        let vértices = [
            Punto::new(1.0, -1.0, 0.0),
            Punto::new(1.0,  1.0, 0.0),
            Punto::new(1.0,  0.0, 1.0)
        ];
        let rayo = Rayo::new(&Punto::new(0.0, 0.0, 0.0), &Vector3::new(1.0, 3.0, 3.0));

        assert!(intersecar_rayo_y_triángulo(&vértices, &rayo).is_none());
    }

    #[test]
    fn caja_interseca_rayo() {
        let caja = Caja::new(&Punto::new(1.0, 1.0, 1.0), &Punto::new(2.0, 2.0, 2.0));
        let rayo = Rayo::new(&Punto::new(0.0, 0.0, 0.0), &Vector3::new(1.5, 1.5, 1.5));

        assert!(caja.intersección(&rayo).is_some());
    }

    #[test]
    fn caja_no_interseca_rayo() {
        let caja = Caja::new(&Punto::new(1.0, 1.0, 1.0), &Punto::new(2.0, 2.0, 2.0));
        let rayo = Rayo::new(&Punto::new(0.0, 0.0, 0.0), &Vector3::new(1.0, 0.0, 0.0));

        assert!(caja.intersección(&rayo).is_none());
    }

    #[test]
    fn rayo_adentro_de_caja() {
        let caja = Caja::new(&Punto::new(0.0, 0.0, 0.0), &Punto::new(2.0, 2.0, 2.0));
        let rayo = Rayo::new(&Punto::new(1.0, 1.0, 1.0), &Vector3::new(1.0, 0.0, 0.0));

        assert!(caja.intersección(&rayo).is_some());
    }

    #[test]
    fn unir_cajas() {
        let c_1 = Caja::new(&Punto::new(1.0, 1.0, 1.0), &Punto::new(2.0, 2.0, 2.0));
        let c_2 = Caja::new(&Punto::new(0.0, 0.0, 0.0), &Punto::new(1.0, 1.0, 1.0));

        let caja = Caja::envolver_cajas(&c_1, &c_2);

        assert!((caja.min.x - 0.0 ).abs() < f64::EPSILON);
        assert!((caja.min.y - 0.0 ).abs() < f64::EPSILON);
        assert!((caja.min.z - 0.0 ).abs() < f64::EPSILON);
        assert!((caja.max.x - 2.0 ).abs() < f64::EPSILON);
        assert!((caja.max.y - 2.0 ).abs() < f64::EPSILON);
        assert!((caja.max.z - 2.0 ).abs() < f64::EPSILON);
    }

    #[test]
    fn ampliar_caja() {
        let mut caja = Caja::new(&Punto::new(1.0, 1.0, 1.0), &Punto::new(2.0, 2.0, 2.0));
        let c_2 = Caja::new(&Punto::new(0.0, 0.0, 0.0), &Punto::new(1.0, 1.0, 1.0));

        caja.ampliar_caja(&c_2);

        assert!((caja.min.x - 0.0 ).abs() < f64::EPSILON);
        assert!((caja.min.y - 0.0 ).abs() < f64::EPSILON);
        assert!((caja.min.z - 0.0 ).abs() < f64::EPSILON);
        assert!((caja.max.x - 2.0 ).abs() < f64::EPSILON);
        assert!((caja.max.y - 2.0 ).abs() < f64::EPSILON);
        assert!((caja.max.z - 2.0 ).abs() < f64::EPSILON);

    }
}

