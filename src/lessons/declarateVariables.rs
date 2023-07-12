fn declarateVariables() {
    let age: u8 = 20;
    // operador "&" se utiliza para tomar una referencia a
    // un valor en lugar de poseerlo directamente. Cuando se utiliza "&str",
    // estás creando una referencia a una cadena de texto (str) existente en
    // lugar de tomar posesión de ella.
    let name: &str = "Luis Alberto";
    println!("Hola soy {} y tengo {} años", name, age);

    // Al parecer las variables tienen que ser escritas en snake_case
    let min_temperature: i8 = -10;
    let max_temperature: u8 = 40;

    println!(
        "La temperatura mínima es {} y la máxima es {}",
        min_temperature, max_temperature
    )
}
