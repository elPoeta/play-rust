struct RGBColor {
    red: u8,
    green: u8,
    blue: u8,
}

impl ToString for RGBColor {
    fn to_string(&self) -> String {
        format!("R: {}, G: {}, B: {}", self.red, self.green, self.blue)
    }
}
struct ColorTuple(u8, u8, u8);

fn main() {
    let mut rgb = RGBColor {
        red: 255,
        green: 32,
        blue: 70,
    };

    rgb.red = 4;
    println!("{}", rgb.to_string());
    // println!("R: {}, G: {}, B: {}", rgb.red, rgb.green, rgb.blue);

    let tuple_color = ColorTuple(67, 21, 10);

    println!("{}", tuple_color.0);
}
