fn loop_lesson() {
    let number_1 = 120;
    let number_2 = 159;

    let sum = number_1 + number_2;

    loop {
        println!("Plase enter de sum of {} and {}", number_1, number_2);

        // get number of result from user
        let mut user_sum_result = String::new();
        std::io::stdin().read_line(&mut user_sum_result).unwrap();

        let user_sum_result_int: i32 = user_sum_result.trim().parse().unwrap();

        if user_sum_result_int == sum {
            println!("You are right!!, congratulations");
            break;
        } else {
            println!(
                "The result {} is not correct, please try again",
                user_sum_result_int
            );
        }
    }
}
