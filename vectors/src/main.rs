fn main() {
    let my_numbers = vec![10, 20, 30, 40];
    let _len = my_numbers.len();

    // my number borrowed else would be dropped if moved
    for num in &my_numbers {
        match num {
            30 => println!("thirty"),
            _ => println!("{}", num),
        }
    }

    println!("The length is = {}", my_numbers.len());
}
