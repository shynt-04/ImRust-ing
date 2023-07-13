// Topic: Enum

enum Color {
    Red,
    Blue,
    Green,
    Yellow,
    Gray
}

fn main() {
    let my_color = Color::Blue;
    match my_color {
        Color::Red => println!("red"),
        Color::Blue => println!("blue"),
        Color::Green => println!("green"),
        Color::Yellow => println!("yellow"),
        Color::Gray => println!("gray"),
    }
}