// Topic: Decision making with match
//
// Program requirements:
// * Display "one", "two", "three", or "other" based on whether
//   the value of a variable is 1, 2, 3, or some other number,
//   respectively
//
// Notes:
// * Use a variable set to any integer
// * Use a match expression to determine which message to display
// * Use an underscore (_) to match on any value

fn one_two_three_other(n: u32) -> &'static str {
    match n {
        1 => "one",
        2 => "two",
        3 => "three",
        _ => "other",
    }
}

fn main() {
    let my_num = one_two_three_other(3);

    println!("my number is... {}",one_two_three_other(1));
    println!("my number is... {}",one_two_three_other(2));
    println!("my number is... {}",my_num);
    println!("my number is... {}",one_two_three_other(100));
}
