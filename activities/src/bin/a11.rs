// Topic: Ownership
//
// Requirements:
// * Print out the quantity and id number of a grocery item
//
// Notes:
// * Use a struct for the grocery item
// * Use two i32 fields for the quantity and id number
// * Create a function to display the quantity, with the struct as a parameter
// * Create a function to display the id number, with the struct as a parameter

struct Item {
    quantity: i32,
    id: i32
}

fn item_quantity(item: &Item) {
    println!("There are {:?} items", item.quantity);
}

fn item_id(item: &Item) {
    println!("The item id is {:?}", item.id);
}

fn main() {
    let my_item = Item {
        quantity: 100,
        id: 2048
    };

    item_quantity(&my_item);
    item_id(&my_item);
}
