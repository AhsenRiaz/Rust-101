fn main() {
   let result = arithmetic(10, 5);
   display_result(result);
}

fn arithmetic(a:i32, b:i32) -> i32 {
    let sum = a + b;
    sum
}

fn display_result(result:i32) {
    println!("The sum is: {}", result);
}

