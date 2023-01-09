struct RGBColor {
    red: u8,
    green: u8,
    blue: u8,
}

fn main() {
    let mut rgb = RGBColor {
        red: 255,
        green: 32,
        blue: 70,
    };

    rgb.red = 4;

    println!("R: {}, G: {}, B: {}", rgb.red, rgb.green, rgb.blue);
}
