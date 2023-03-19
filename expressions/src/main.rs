#[allow(dead_code)]
enum Access {
    Admin,
    Guest,
    Manager,
    User,
}
fn main() {
    handle_access();
    handle_variable();
}

fn handle_access() {
    let access_level = Access::Guest;

    let _can_access_file = match access_level {
        Access::Admin => {
            println!("Can access File");
            true
        }
        _ => {
            println!("Cannot access file");
            false
        }
    };
}

fn handle_variable() {
    let value = 101;
    let is_gt_100 = value > 100;

    match is_gt_100 {
        true => println!("Its big"),
        false => println!("Its small"),
    }
}
