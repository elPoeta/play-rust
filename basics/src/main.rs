fn main() {
    let greet = "Hello";
    let name = "elPoeta";
    println!("Hello, world!");
    println!("Hello {:?}",name);
    println!("{:?} {:?}",greet, name);
    
    let mut count = 0;
    loop {
        if count == 5 {
            break;
        }
        println!("{:?}",count);
        count+= 1;
    }
}
