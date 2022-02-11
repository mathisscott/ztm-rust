// Topic: Testing
//
// Requirements:
// * Write tests for the existing program to ensure proper functionality.
//
// Notes:
// * Create at least two test cases for each function.
// * Use `cargo test` to test the program.
// * There are intentional bugs in the program that need to be fixed.
//   * Check the documentation comments for the functions to
//     determine how the they should operate.

/// Ensures n is >= lower and <= upper.
fn clamp(n: i32, a: i32, b: i32) -> i32 {
    let lower: i32;
    let upper: i32;

    if a < b {
        lower = a;
        upper = b;
    } else if a > b {
        lower = b;
        upper = a;
    } else {
        return a;
    }

    if n < lower {
        lower
    } else if n > upper {
        upper
    } else {
        n
    }
}

/// Divides a and b.
fn div(a: i32, b: i32) -> Option<f64> {
    if b == 0 {
        return None;
    }
    let float_a = f64::from(a);
    let float_b = f64::from(b);
    Some(float_a / float_b)
}

/// Takes two strings and places them immediately one after another.
fn concat(first: &str, second: &str) -> String {
    format!("{}{}", first, second)
}

fn main() {}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn test_clamp() {
        assert_eq!(clamp(1, -2, 2), 1, "mid value returned");
        assert_eq!(clamp(2, -2, 2), 2, "high value returned");
        assert_eq!(clamp(-2, -2, 2), -2, "low value returned");
        assert_eq!(clamp(4, -2, 2), 2, "high value clamped");
        assert_eq!(clamp(-8, -2, 2), -2, "low value clamped");
        assert_eq!(clamp(4, 2, -2), 2, "switching up numbers doesn't matter");
        assert_eq!(clamp(4, 2, 2), 2, "passing same numbers forces return");
    }

    #[test]
    fn test_div() {
        assert_eq!(div(1, 4), Some(0.25), "fractional works");
        assert_eq!(div(8, 2), Some(4.0), "whole works");
        assert_eq!(div(8, 0), None, "no divide by zero");
    }

    #[test]
    fn test_concat() {
        assert_eq!(concat("hello", "world"), "helloworld", "it works");
    }
}