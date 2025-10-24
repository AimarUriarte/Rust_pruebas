use std::io;

struct Vehiculo {
    marca: String,
    anio: i32,
    estado: Estado,
}

enum Estado {
    Nuevo,
    Usado,
    Chatarra
}
fn main() {
    let vehiculo1 = Vehiculo {
        marca: String::from("BMW"),
        anio: 2016,
        estado: Estado::Nuevo,
    };

    let vehiculo2 = Vehiculo {
        marca: String::from("Audi"),
        anio: 2006,
        estado: Estado::Chatarra
    };

    println!("Introduce la marca del vehículo:");
    let mut marca = String::new();
    io::stdin().read_line(&mut marca).expect("Error al leer la marca");
    let marca = marca.trim().to_string();

    println!("Introduce el año del vehículo:");
    let mut anio_str = String::new();
    io::stdin().read_line(&mut anio_str).expect("Error al leer el año");
    let anio: i32 = anio_str.trim().parse().expect("Introduce un número válido");

    println!("Introduce el estado del vehículo (nuevo/usado/chatarra):");
    let mut estado_str = String::new();
    io::stdin().read_line(&mut estado_str).expect("Error al leer el estado");
    let estado_str = estado_str.trim().to_lowercase();

    let estado = match estado_str.as_str() {
        "nuevo" => Estado::Nuevo,
        "usado" => Estado::Usado,
        "chatarra" => Estado::Chatarra,
        _ => {
            println!("Estado no válido, se usará 'Usado' por defecto.");
            Estado::Usado
        }
    };

    let vehiculo3 = Vehiculo {
        marca,
        anio,
        estado,
    };
    mostrar_info(&vehiculo1);
    mostrar_info(&vehiculo2);
    mostrar_info(&vehiculo3);

}

fn mostrar_info(v: &Vehiculo) {
    match v.estado {
        Estado::Nuevo => println!("Marca: {}, año: {} , Estado: nuevo.", v.marca, v.anio),
        Estado::Usado => println!("Marca: {}, año: {} , Estado: usado.", v.marca, v.anio),
        Estado::Chatarra => println!("Marca: {}, año: {} , Estado: chatarra.", v.marca, v.anio),
    }
}