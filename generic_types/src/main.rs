#[allow(dead_code)]
struct Point<T, U> {
    x: T,
    y: U,
}

struct Dimensions<A> {
    width: A,
    height: A,
}

impl<A> Dimensions<A> {
    fn x(&self) -> &A {
        &self.width
    }
}

impl Dimensions<f64> {
    fn y(&self) -> &f64 {
        &self.height
    }
}

fn main() {
    let number_list = vec![10, 30, 39, 50, 79, 100];
    let largest = get_largest(number_list);
    println!("{}", largest);

    let char_list = vec!['a', 'b'];
    let largest_char = get_largest(char_list);
    println!("{}", largest_char);

    let _p1 = Point { x: 'a', y: 4 };
    let _p2 = Point { x: 1.3, y: 4 };
    let _p3 = Point { x: 10, y: 20 };

    let d1 = Dimensions {
        width: 1,
        height: 5,
    };

    d1.x(); // y() not available as the type is generict, not f64

    let d2 = Dimensions {
        width: 1.5,
        height: 4.3,
    };

    d2.y();
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
