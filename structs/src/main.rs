enum Flavour {
    Sparkling,
    Sweet,
    Fruity,
}

#[allow(dead_code)]
struct Drink {
    flavor: Flavour,
    fluid_oz: f64,
}

fn print_drink(drink: Drink) {
    match drink.flavor {
        Flavour::Fruity => println!("fruity flavor"),
        Flavour::Sparkling => println!("sparkling flavor"),
        Flavour::Sweet => println!("sweet flavor"),
    }
}

fn main() {
    let sweet = Drink {
        flavor: Flavour::Sweet,
        fluid_oz: 1.15,
    };

    print_drink(sweet);

    let fruity = Drink {
        flavor: Flavour::Fruity,
        fluid_oz: 1.15,
    };
    print_drink(fruity);

    let sparkling = Drink {
        flavor: Flavour::Sparkling,
        fluid_oz: 1.15,
    };

    print_drink(sparkling);
}
