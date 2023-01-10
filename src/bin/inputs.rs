use std::io;

fn main() {
    println!("enter your name: ");
    let mut user_name = String::new();

    io::stdin()
        .read_line(&mut user_name)
        .expect("Failed to read line");

    let mut age = String::new();
    println!("Enter your age: ");

    io::stdin()
        .read_line(&mut age)
        .expect("Failed to read line");

    let age: u32 = age.trim().parse().unwrap();

    println!(
        "My name is {} and i have {} years old.",
        user_name.trim(),
        age
    );
}
