struct RGBColor {
    red: u8,
    green: u8,
    blue: u8,
}

struct ColorTuple(u8, u8, u8);

fn main() {
    let mut rgb = RGBColor {
        red: 255,
        green: 32,
        blue: 70,
    };

    rgb.red = 4;

    println!("R: {}, G: {}, B: {}", rgb.red, rgb.green, rgb.blue);

    let tuple_color = ColorTuple(67, 21, 10);

    println!("{}", tuple_color.0);
}
