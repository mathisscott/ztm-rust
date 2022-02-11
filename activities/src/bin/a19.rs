use std::collections::HashMap;

// Topic: HashMap
//
// Requirements:
// * Print the name and number of items in stock for a furniture store
// * If the number of items is 0, print "out of stock" instead of 0
// * The store has:
//   * 5 Chairs
//   * 3 Beds
//   * 2 Tables
//   * 0 Couches
// * Print the total number of items in stock
//
// Notes:
// * Use a HashMap for the furniture store stock

fn main() {
    let mut inventory = HashMap::new();
    let mut total_inv = 0;
 
    inventory.insert("Chair", 5);
    inventory.insert("Bed", 3);
    inventory.insert("Endtable", 1);
    inventory.insert("Table", 2);
    inventory.insert("Couch", 0);

    for (item, qty) in inventory.iter() {
        if qty == &1 {
            println!("{:?} {:?} in stock!", qty, item);
            total_inv = total_inv + 1;
        } else if qty > &1 {
            println!("{:?} {:?}s in stock!", qty, item);
            total_inv = total_inv + qty;
        } else {
            println!("{:?} out of stock", item);
        }
    }

    println!("Total items: {:?}", total_inv);
}
