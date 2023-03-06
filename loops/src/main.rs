fn main() {
    print_five_times();
    print_three_times();
    countdown();
}

// write program to print a name 5 times;

fn print_five_times() {
    let mut i = 4;
    loop {
        println!("My name is ahsen");
        i -= 1;

        if i < 0 {
            break;
        }
    }
}

// write a program to print i 3 times and break after 3rd loop

fn print_three_times() {
    let mut i = 1;

    while i <= 3 {
        println!("{}", i);
        i += 1;
    }
}

// write a countdown program and print done when completed

fn countdown() {
    let mut i = 5;

    while i >= 1 {
        println!("{}", i);
        i -= 1;
    }

    println!("Done");
}
