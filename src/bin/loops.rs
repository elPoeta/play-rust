fn main() {
    println!("Start!");
    let mut counter = 0;
    loop {
        if counter == 5 {
            println!("Well Done! {:?}", counter);
            break;
        }
        counter += 1;
    }

    while counter > 0 {
        println!("{:?}", counter);
        counter -= 1;
    }

    println!("End!");
}
