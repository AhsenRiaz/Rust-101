fn main() {
    let number_list = vec![10, 30, 39, 50, 79, 100];
    let largest = get_largest(number_list);
    println!("{}", largest);

    let char_list = vec!['a', 'b'];
    let largest_char = get_largest(char_list);
    println!("{}", largest_char);
}

fn get_largest<T: PartialOrd + Copy>(number_list: Vec<T>) -> T {
    let mut max = number_list[0];

    for number in number_list {
        if number > max {
            max = number;
        }
    }

    max
}
