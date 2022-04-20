pub fn menor_de_tres(a: f64, b: f64, c: f64) -> f64 {
    if a < b && a < c {
        a
    } else if b < a && b < c {
        b
    } else {
        c
    }
}

pub fn mayor_de_tres(a: f64, b: f64, c: f64) -> f64 {
    if a > b && a > c {
        a
    } else if b > a && b > c {
        b
    } else {
        c
    }
}

pub fn leer_archivo(nombre: &str) -> Result<String, anyhow::Error> {
    use std::fs::File;
    use std::io::prelude::*;

    let mut archivo = File::open(nombre)
        .map_err(|_| anyhow::anyhow!("Archivo {:?} no encontrado.", nombre))?;
    let mut texto = String::new();

    archivo.read_to_string(&mut texto)
        .map_err(|err| anyhow::anyhow!("Error leyendo el archivo.\n{:?}", err))?;

    Ok(texto)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_menor_de_tres() {
        let (a, b, c) = (1.0, 2.0, 3.0);

        assert!(menor_de_tres(a, b, c) - a < 1e-10);
        assert!(menor_de_tres(c, b, a) - a < 1e-10);
        assert!(menor_de_tres(b, a, c) - a < 1e-10);
    }

    #[test]
    fn test_mayor_de_tres() {
        let (a, b, c) = (1.0, 2.0, 3.0);

        assert!(mayor_de_tres(a, b, c) - c < 1e-10);
        assert!(mayor_de_tres(c, b, a) - c < 1e-10);
        assert!(mayor_de_tres(b, c, a) - c < 1e-10);
    }
}

