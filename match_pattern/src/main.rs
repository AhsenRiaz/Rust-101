use std::io;

fn main() {
    println!("Enter the name");

    let mut name = String::new();

    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");

    match name.trim() {
        "Bob" => println!("Hello Bob"),
        "Jayson" => println!("Hi Jayson! What are you doing here"),
        _ => println!("Nice to meet you all"),
    }
}
