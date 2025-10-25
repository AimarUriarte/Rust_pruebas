mod usuarios;
use std::io::{self, Write};

fn main() {
    loop {
        println!("\n--- MINI CRUD DE USUARIOS ---");
        println!("1. Agregar usuario");
        println!("2. Listar usuarios");
        println!("3. Buscar usuario");
        println!("4. Modificar edad");
        println!("5. Eliminar usuario");
        println!("6. Salir");
        print!("Selecciona una opción: ");
        io::stdout().flush().unwrap();

        let mut opcion = String::new();
        io::stdin().read_line(&mut opcion).unwrap();
        let opcion = opcion.trim();

        match opcion {
            "1" => {
                let (nombre, edad) = pedir_datos();
                usuarios::agregar_usuario(&nombre, edad);
            },
            "2" => usuarios::listar_usuarios(),
            "3" => {
                let nombre = pedir_nombre();
                match usuarios::buscar_usuario(&nombre) {
                    Some(u) => println!("Usuario encontrado: {} ({} años)", u.nombre, u.edad),
                    None => println!("Usuario '{}' no encontrado.", nombre),
                }
            },
            "4" => {
                let nombre = pedir_nombre();
                let edad = pedir_edad();
                usuarios::modificar_edad(&nombre, edad);
            },
            "5" => {
                let nombre = pedir_nombre();
                usuarios::eliminar_usuario(&nombre);
            },
            "6" => {
                println!("Saliendo...");
                break;
            },
            _ => println!("Opción no válida."),
        }
    }
}

// Funciones auxiliares para pedir datos al usuario
fn pedir_datos() -> (String, u8) {
    let nombre = pedir_nombre();
    let edad = pedir_edad();
    (nombre, edad)
}

fn pedir_nombre() -> String {
    print!("Introduce el nombre: ");
    io::stdout().flush().unwrap();
    let mut nombre = String::new();
    io::stdin().read_line(&mut nombre).unwrap();
    nombre.trim().to_string()
}

fn pedir_edad() -> u8 {
    loop {
        print!("Introduce la edad: ");
        io::stdout().flush().unwrap();
        let mut edad_str = String::new();
        io::stdin().read_line(&mut edad_str).unwrap();
        match edad_str.trim().parse::<u8>() {
            Ok(n) => return n,
            Err(_) => println!("Edad no válida, intenta de nuevo."),
        }
    }
}
