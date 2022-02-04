// Topic: Basic arithmetic
//
// Program requirements:
// * Displays the result of the sum of two numbers
//
// Notes:
// * Use a function to add two numbers together
// * Use a function to display the result
// * Use the "{:?}" token in the println macro to display the result

fn sum(a: i32, b: i32) -> i32 {
    a + b
}

fn display_sum(sum: i32) {
    println!("sum is... {:?}", sum);
}

fn main() {
    let result = sum(40, 1);
    display_sum(result);
}
