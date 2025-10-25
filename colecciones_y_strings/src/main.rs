use std::collections::HashMap;
use std::io;

fn main() {
    let mut entrada = String::new();
    println!("Escribe una frase:");
    io::stdin().read_line(&mut entrada).expect("Error");

    let mut contador = HashMap::new();

    for palabra in entrada.split_whitespace() {
        let palabra = palabra.to_lowercase();
        *contador.entry(palabra).or_insert(0) += 1;
    }

    println!("\nConteo de palabras:");
    for (palabra, cantidad) in contador {
        println!("{}: {}", palabra, cantidad);
    }
}
