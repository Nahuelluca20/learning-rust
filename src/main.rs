fn main() {
    // Obtener el nombre
    println!("Enter your name:");
    let mut name: String = String::new();

    // std: es la libreria estandar de rust que se contecta con el sistema operativo
    // io: es la libreria de entrada y salida de datos
    // stdin: le dice a la maquina que se debe de pedir datos al usuario
    // read_line: lee una linea de texto
    // &mut name: esto indica que los datos se guardan en name
    // unwrap: es un método que se utiliza para desempaquetar el valor 
    // contenido dentro de un Option o Result.
    std::io::stdin().read_line(&mut name).unwrap();
    name = name.trim().to_string();

    // Obtener la edad y convertirlo a string
    println!("Enter your age:");
    let mut age: String = String::new();
    std::io::stdin().read_line(&mut age).unwrap();
    let age_int: u8 = age.trim().parse().unwrap();

    println!("Hello, Welcome {} of {} years old", name, age_int);
}


