// Topic: Flow control using if..else if..else
//
// Program requirements:
// * Display ">5", "<5", or "=5" based on the value of a variable
//   is > 5, < 5, or == 5, respectively
//
// Notes:
// * Use a variable set to any integer value
// * Use an if..else if..else block to determine which message to display
// * Use the println macro to display messages to the terminal

fn relation_to_five(num: i32) -> &'static str {
    let return_str;
    if num > 5 {
        return_str = ">5";
    } else if num < 5 {
        return_str = "<5";
    } else {
        return_str = "=5";
    }
    return_str
}

fn main() {
    let test = 6;
    println!("{} is {}", test, relation_to_five(test));
}
