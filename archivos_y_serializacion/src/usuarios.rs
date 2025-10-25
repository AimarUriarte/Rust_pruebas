use serde::{Serialize, Deserialize};
use serde_json;
use std::fs;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Usuario {
    pub nombre: String,
    pub edad: u8,
}

const ARCHIVO: &str = "usuarios.json";


pub fn agregar_usuario(nombre: &str, edad: u8) {
    let mut usuarios = leer_usuarios();

    usuarios.push(Usuario {
        nombre: nombre.to_string(),
        edad,
    });

    guardar_usuarios(&usuarios);
    println!("Usuario '{}' agregado correctamente.", nombre);
}

pub fn listar_usuarios() {
    let usuarios = leer_usuarios();

    if usuarios.is_empty() {
        println!("No hay usuarios registrados.");
        return;
    }

    println!("Lista de usuarios:");
    for (i, u) in usuarios.iter().enumerate() {
        println!("{}. {} ({} años)", i + 1, u.nombre, u.edad);
    }
}

pub fn buscar_usuario(nombre: &str) -> Option<Usuario> {
    let usuarios = leer_usuarios();
    usuarios.into_iter().find(|u| u.nombre.to_lowercase() == nombre.to_lowercase())
}

pub fn eliminar_usuario(nombre: &str) {
    let mut usuarios = leer_usuarios();
    let original_len = usuarios.len();

    usuarios.retain(|u| u.nombre.to_lowercase() != nombre.to_lowercase());
    guardar_usuarios(&usuarios);

    if usuarios.len() < original_len {
        println!("Usuario '{}' eliminado.", nombre);
    } else {
        println!("Usuario '{}' no encontrado.", nombre);
    }
}

pub fn modificar_edad(nombre: &str, nueva_edad: u8) {
    let mut usuarios = leer_usuarios();
    let mut encontrado = false;

    for u in usuarios.iter_mut() {
        if u.nombre.to_lowercase() == nombre.to_lowercase() {
            u.edad = nueva_edad;
            encontrado = true;
            println!("Edad de '{}' actualizada a {} años.", nombre, nueva_edad);
            break;
        }
    }

    if !encontrado {
        println!("Usuario '{}' no encontrado.", nombre);
    }

    guardar_usuarios(&usuarios);
}


fn leer_usuarios() -> Vec<Usuario> {
    match fs::read_to_string(ARCHIVO) {
        Ok(contenido) => serde_json::from_str(&contenido).unwrap_or_else(|_| Vec::new()),
        Err(_) => Vec::new(),
    }
}

fn guardar_usuarios(usuarios: &Vec<Usuario>) {
    let json = serde_json::to_string_pretty(usuarios).expect("Error al convertir a JSON");
    fs::write(ARCHIVO, json).expect("No se pudo escribir en el archivo");
}
