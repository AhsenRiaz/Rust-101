enum Colors {
    Red,
    Blue,
    Green,
    Yellow,
}

fn main() {
    print_color(Colors::Red);
}

fn print_color(color: Colors) {
    match color {
        Colors::Blue => println!("blue color"),
        Colors::Red => println!("red color"),
        Colors::Green => println!("green color"),
        Colors::Yellow => println!("yellow color"),
    }
}
