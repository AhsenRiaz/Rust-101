fn main() {
    let dimensions = Dimensions {
        width: 10,
        height: 20,
        depth: 30,
    };

    let shipping_box = ShippingBox::new(Color::Red, 0.5, dimensions);

    shipping_box.print();
}

enum Color {
    Brown,
    Red,
}

impl Color {
    fn print(&self) {
        match self {
            Color::Brown => println!("Brown"),
            Color::Red => println!("Red"),
        }
    }
}

struct Dimensions {
    width: u32,
    height: u32,
    depth: u32,
}

impl Dimensions {
    fn print(&self) {
        println!("width: {}", &self.width);
        println!("height: {}", &self.height);
        println!("depth: {}", &self.depth);
    }
}

struct ShippingBox {
    color: Color,
    weight: f64,
    dimensions: Dimensions,
}

impl ShippingBox {
    fn new(color: Color, weight: f64, dimensions: Dimensions) -> Self {
        Self {
            color,
            weight,
            dimensions,
        }
    }

    fn print(&self) {
        self.color.print();
        println!("weight: {}", self.weight);
        self.dimensions.print();
    }
}
