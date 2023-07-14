use regex::Regex;
fn addOperation() {
    println!("Hello, world!");

    // Regex
    let re_add = Regex::new(r"(\d+)\s?\+\s?(\d+)").unwrap();
    let re_mult = Regex::new(r"(\d+)\s?\*\s?(\d+)").unwrap();

    // get data from user
    println!("Prease, enter your expression");
    let mut expression = String::new();
    std::io::stdin().read_line(&mut expression).unwrap();

    loop {
        let caps = re_add.captures(expression.as_str());

        if caps.is_none() {
            break;
        }

        let caps = caps.unwrap();

        let caps_expression: &str = caps.get(0).unwrap().as_str();
        let left_value: i32 = caps.get(1).unwrap().as_str().parse::<i32>().unwrap();
        let right_value: i32 = caps.get(2).unwrap().as_str().parse::<i32>().unwrap();

        let addition: i32 = left_value + right_value;

        expression = expression.replace(caps_expression, &addition.to_string());
    }

    // Operations

    // show result
    println!("Result: \n{} ", expression);
}
