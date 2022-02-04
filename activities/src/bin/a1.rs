// Topic: Functions
//
// Program requirements:
// * Displays your first and last name
//
// Notes:
// * Use a function to display your first name
// * Use a function to display your last name
// * Use the println macro to display messages to the terminal

fn first_name() -> &'static str {
    "Scott"
}

fn last_name() -> &'static str {
    "Mathis"
}

fn main() {
    println!("{} {}", first_name(), last_name());
}
