fn main() {
    let str = String::from("Learning Rust lang");

    println!("Length: {}", str.len());

    println!("Is Empty? {}", str.is_empty());

    for token in str.split_whitespace() {
        println!("Token: {:?}", token);
    }

    let characters = str.chars();

    for c in characters {
        if c.is_ascii_whitespace() {
            continue;
        }
        println!("{}", c);
    }
}
