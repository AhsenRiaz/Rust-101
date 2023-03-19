struct Person {
    name: String,
    fav_color: String,
    age: i32,
}

fn main() {
    let people = vec![
        Person {
            name: String::from("Alice"),
            fav_color: String::from("Blue"),
            age: 18,
        },
        Person {
            name: String::from("Bob"),
            fav_color: String::from("Red"),
            age: 20,
        },
    ];

    for person in people {
        if person.age >= 18 {
            print(&person.name);
            print(&person.fav_color);
        }
    }
}

fn print(data: &str) {
    println!("{}", data);
}
