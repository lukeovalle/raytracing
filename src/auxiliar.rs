pub fn smaller_of_three(a: f64, b: f64, c: f64) -> f64 {
    if a < b && a < c {
        a
    } else if b < a && b < c {
        b
    } else {
        c
    }
}

pub fn bigger_of_three(a: f64, b: f64, c: f64) -> f64 {
    if a > b && a > c {
        a
    } else if b > a && b > c {
        b
    } else {
        c
    }
}

#[macro_export]
macro_rules! assert_eq_float {
    ($left:expr, $right:expr) => {
        assert!(
            ($left - $right).abs() < 1e-10,
        )
    }
}

#[macro_export]
macro_rules! assert_eq_vec {
    ($left:expr, $right:expr) => {
        assert_eq_float!($left.x, $right.x);
        assert_eq_float!($left.y, $right.y);
        assert_eq_float!($left.z, $right.z);
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smaller_of_three() {
        let (a, b, c) = (1.0, 2.0, 3.0);

        assert!(smaller_of_three(a, b, c) - a < 1e-10);
        assert!(smaller_of_three(c, b, a) - a < 1e-10);
        assert!(smaller_of_three(b, a, c) - a < 1e-10);
    }

    #[test]
    fn test_bigger_of_three() {
        let (a, b, c) = (1.0, 2.0, 3.0);

        assert!(bigger_of_three(a, b, c) - c < 1e-10);
        assert!(bigger_of_three(c, b, a) - c < 1e-10);
        assert!(bigger_of_three(b, c, a) - c < 1e-10);
    }
}
