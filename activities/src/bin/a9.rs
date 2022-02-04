// Topic: Data management using tuples
//
// Requirements:
// * Print whether the y-value of a cartesian coordinate is
//   greater than 5, less than 5, or equal to 5
//
// Notes:
// * Use a function that returns a tuple
// * Destructure the return value into two variables
// * Use an if..else if..else block to determine what to print

fn coords() -> (u32, u32) {
    (100, 50)
}

fn main() {
    let (_, y) = coords();

    if y > 5 {
        println!("Y is greater than 5!");
    } else if y < 5 {
        println!("Y is less than 5!");
    } else {
        println!("Y is actually 5!");
    }
}
