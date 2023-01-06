enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn witch_way(go: Direction) {
    match go {
        Direction::Up => println!("UP!"),
        Direction::Down => println!("DOWN!"),
        Direction::Left => println!("LEFT!"),
        Direction::Right => println!("RIGHT!"),
    }
}
fn main() {
    let go = Direction::Left;
    witch_way(go);
}
