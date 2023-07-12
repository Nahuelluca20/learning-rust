fn conditionals() {
    // Obtener la edad y convertirlo a string
    println!("Enter your age:");

    let mut age: String = String::new();
    std::io::stdin().read_line(&mut age).unwrap();

    let age_int: u8 = age.trim().parse().unwrap();

    if age_int >= 18 && age_int != 30 {
        println!("You can enter to discotheque");

        if age_int > 70 {
            println!("You should moderate your alcohol consumption");
        }
    } else if age_int == 30 {
        println!("You can enter to discotheque");
    } else {
        println!("You can't enter to discotheque");
    }

    println!("You have {} years old", age_int);
}
