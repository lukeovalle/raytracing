#[inline]
pub fn smaller_of_three(a: f64, b: f64, c: f64) -> f64 {
    a.min(b.min(c))
}

#[inline]
pub fn bigger_of_three(a: f64, b: f64, c: f64) -> f64 {
    a.max(b.max(c))
}

/// Interpolación lineal entre a y b
#[inline]
pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    (1.0 - t) * a + t * b
}

pub fn read_file(nombre: &str) -> Result<String, anyhow::Error> {
    use std::fs::File;
    use std::io::prelude::*;

    let mut archivo = File::open(nombre).map_err(|_| {
        anyhow::anyhow!("Archivo \"{:?}\" no encontrado.", nombre)
    })?;
    let mut texto = String::new();

    archivo.read_to_string(&mut texto).map_err(|err| {
        anyhow::anyhow!("Error leyendo el archivo.\n{:?}", err)
    })?;

    Ok(texto)
}

#[macro_export]
macro_rules! assert_eq_float {
    ($left:expr, $right:expr) => {
        assert!(($left - $right).abs() < 1e-10,)
    };
}

#[macro_export]
macro_rules! assert_eq_vec {
    ($left:expr, $right:expr) => {
        assert_eq_float!($left.x, $right.x);
        assert_eq_float!($left.y, $right.y);
        assert_eq_float!($left.z, $right.z);
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smaller_of_three() {
        let (a, b, c) = (1.0, 2.0, 3.0);

        assert_eq_float!(smaller_of_three(a, b, c), a);
        assert_eq_float!(smaller_of_three(c, b, a), a);
        assert_eq_float!(smaller_of_three(b, a, c), a);
    }

    #[test]
    fn test_bigger_of_three() {
        let (a, b, c) = (1.0, 2.0, 3.0);

        assert_eq_float!(bigger_of_three(a, b, c), c);
        assert_eq_float!(bigger_of_three(c, b, a), c);
        assert_eq_float!(bigger_of_three(b, c, a), c);
    }
}
