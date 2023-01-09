struct Point {
    x: u32,
    y: u32,
}

impl ToString for Point {
    fn to_string(&self) -> String {
        format!("Point(X: {}, Y: {})", self.x, self.y)
    }
}

fn main() {
    let point = Point { x: 10, y: 20 };

    print_point(&point);
    print_point(&point);

    let mut n = 20;

    {
        let shadow = &mut n;

        *shadow += 1;
    }

    println!("N: {}", n);
}

fn print_point(p: &Point) {
    println!("{}", p.to_string());
}
