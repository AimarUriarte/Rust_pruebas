use rand::*;
use std::sync::Mutex;
pub struct Usuario {
    pub nombre: String,
    pub edad: u8,
}
lazy_static::lazy_static! {
    static ref USUARIOS: Mutex<Vec<Usuario>> = Mutex::new(Vec::new());
}

pub fn registrar(nombre: &str) {
    let edad = 20 + random::<u8>() % 21;
    let nuevo_usuario = Usuario {
        nombre: nombre.to_string(),
        edad: edad,
    };

    let mut usuarios = USUARIOS.lock().unwrap();
    usuarios.push(nuevo_usuario);
}

pub fn listar() {
    let usuarios = USUARIOS.lock().unwrap();
    println!("Lista de usuarios registrados:");
    for (i, u) in usuarios.iter().enumerate() {
        println!("{}. {} ({} años)", i + 1, u.nombre, u.edad);
    }
}

pub fn modificar_edad(nombre: &str, nueva_edad: u8) {
    let mut usuarios = USUARIOS.lock().unwrap();
    let mut encontrado = false;

    for usuario in usuarios.iter_mut() { // se usa iter_mut porque quiero cambiar algo dentro del vector
        if usuario.nombre.to_lowercase() == nombre.to_lowercase() {
            usuario.edad = nueva_edad;
            encontrado = true;
            println!("Edad de '{}' actualizada a {} años.", nombre, nueva_edad);
            break;
        }
    }

    if !encontrado {
        println!("Usuario '{}' no encontrado.", nombre);
    }
}

pub fn eliminar(nombre: &str) {
    let mut usuarios = USUARIOS.lock().unwrap();
    let original_len = usuarios.len();

    // Con retain se retiene solo los usuarios que no coincidan con el nombre
    usuarios.retain(|u| u.nombre.to_lowercase() != nombre.to_lowercase());

    if usuarios.len() < original_len {
        println!("Usuario '{}' eliminado.", nombre);
    } else {
        println!("Usuario '{}' no encontrado.", nombre);
    }
}

