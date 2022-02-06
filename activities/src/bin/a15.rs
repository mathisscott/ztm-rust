// Topic: Advanced match
//
// Requirements:
// * Print out a list of tickets and their information for an event
// * Tickets can be Backstage, Vip, and Standard
// * Backstage and Vip tickets include the ticket holder's name
// * All tickets include the price
//
// Notes:
// * Use an enum for the tickets with data associated with each variant
// * Create one of each ticket and place into a vector
// * Use a match expression while iterating the vector to print the ticket info

enum Ticket {
    Backstage(f64, String),
    Standard(f64),
    Vip(f64, String),
}

fn main() {
    let tix: Vec<Ticket> = vec![
        Ticket::Backstage(200.0, "Ted".to_owned()),
        Ticket::Standard(80.0),
        Ticket::Vip(400.0, "Bill".to_owned()),
    ];

    for t in tix {
        match t {
            Ticket::Standard(price) => println!("standard ticket - price: {:?}", price),
            Ticket::Backstage(price, customer_name) => println!("backstage ticket - name: {:?}, price: {:?}", customer_name, price),
            Ticket::Vip(price, customer_name) => println!("vip ticket - name: {:?}, price: {:?}", customer_name, price),
        }
    }
}
