struct Temperature {
    degree_f: i32,
}

impl Temperature {
    fn freezing() -> Self {
        Self { degree_f: -273 }
    }

    fn show_temperature(&self) {
        println!("{}°C", self.degree_f);
    }
}

fn main() {
    let hot = Temperature { degree_f: 99 };
    hot.show_temperature();

    let cold = Temperature::freezing();
    cold.show_temperature();
}
