use std::fs::File;
use std::io::{self, Read};

fn leer_archivo() -> io::Result<Vec<i32>> {
    let mut contenido = String::new();
    let mut archivo = match File::open("datos.txt") {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error: no se pudo abrir el archivo: {}", e);
            return Err(e);
        }
    };
    if let Err(e) = archivo.read_to_string(&mut contenido) {
        eprintln!("Error al leer el archivo: {}", e);
        return Err(e);
    }

    let mut numeros: Vec<i32> = Vec::new();
    for (i, linea) in contenido.lines().enumerate() {
        match linea.trim().parse::<i32>() {
            Ok(num) => numeros.push(num),
            Err(_) => {
                eprintln!("Línea {} no es un número válido: '{}'", i + 1, linea);
            }
        }
    }

    Ok(numeros)
}

fn main() -> std::io::Result<()>{
    let numeros = leer_archivo()?;
    if numeros.is_empty() {
        println!("No hay datos válidos para calcular el promedio.");
        return Ok(());
    }
    println!("{:?}", numeros);
    let suma: i32 = numeros.iter().sum();
    let promedio:f32 = suma as f32 / numeros.len() as f32;
    println!("el promedio es: {}", promedio);
    Ok(())
}