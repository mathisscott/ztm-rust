// Topic: Working with expressions
//
// Requirements:
// * Print "its big" if a variable is > 100
// * Print "its small" if a variable is <= 100
//
// Notes:
// * Use a boolean variable set to the result of
//   an if..else expression to store whether the value
//   is > 100 or <= 100
// * Use a function to print the messages
// * Use a match expression to determine which message
//   to print

fn print_msg(is_big: bool) {
    println!("{:?}", match is_big {
        true => "it's big",
        false => "it's small",
    });
}

fn main() {
    let size = 15;
    let size_over_100 = size > 100;
    print_msg(size_over_100);
}
