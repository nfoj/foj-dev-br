use std::io;

pub fn cfconstructs() {
    let mut name = String::new();

    println!("Enter your name: ");
    io::stdin()
        .read_line(&mut name)
        .expect("Failed in read line!");

    println!("Hello, {}!", name.trim());

    println!("Enter you age: ");
    let mut age = String::new();
    io::stdin()
        .read_line(&mut age)
        .expect("Failed in read line!");

    let age: i8 = age.trim().parse().expect("Failed in read line!");

    println!("You are {} years.", age);
}
