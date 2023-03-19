struct GroceryItem {
    quantity: i32,
    id: i32,
}

fn display_quantity(item: &GroceryItem) {
    println!("{}", item.quantity);
}

fn display_id(item: GroceryItem) {
    println!("{}", item.id);
}

fn main() {
    let my_item = GroceryItem { quantity: 1, id: 1 };

    display_quantity(&my_item);

    display_id(my_item);
}
