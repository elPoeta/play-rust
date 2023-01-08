const MAXIMUN_NUMBER: u8 = 20;

fn main() {
    let numbers = 0..MAXIMUN_NUMBER;

    for n in numbers {
        println!("Number: {}", n);
    }
}
