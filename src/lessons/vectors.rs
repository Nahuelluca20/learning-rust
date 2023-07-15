fn vectors() {
  let mut names: Vec<String> = Vec::new();

  for _i in 0..3 {
      let mut name: String = String::new();

      println!("Please enter your name");
      std::io::stdin().read_line(&mut name).unwrap();

      names.push(name);
  }

  println!("{:?}", names);
  println!("{}", names[0]);
  println!("{}", names.len());

  for name in names {
      println!("The name is {}", name);
  }

  let hello = ["H", "e", "l", "l", "o"];
  println!("{}", hello[0]);

}
