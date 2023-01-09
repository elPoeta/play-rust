fn main() {
    let x = 5;
    {
        let x = 20;
        if x > 10 {
            println!("{} > 10", x);
        }
    }

    println!("{}", x);
}
