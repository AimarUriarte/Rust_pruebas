mod usuarios;
use std::io;
fn main() {
    usuarios::registrar("Ana");
    usuarios::registrar("Luis");
    usuarios::registrar("Marta");

    usuarios::listar();

    let mut nombre = String::new();
    let mut edad_str = String::new();

    println!("Introduce el nombre del usuario cuya edad quieres modificar:");
    io::stdin().read_line(&mut nombre).unwrap();
    let nombre = nombre.trim();

    println!("Introduce la nueva edad:");
    io::stdin().read_line(&mut edad_str).unwrap();
    let nueva_edad: u8 = match edad_str.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Edad no válida.");
            return;
        }
    };

    usuarios::modificar_edad(nombre, nueva_edad);

    println!("\nUsuarios actualizados:");
    usuarios::listar();

    let mut nombre = String::new();
    println!("Introduce el nombre del usuario que quieres eliminar:");
    io::stdin().read_line(&mut nombre).unwrap();
    let nombre = nombre.trim();
    usuarios::eliminar(nombre);
    
    println!("\nUsuarios actualizados:");
    usuarios::listar();
}
