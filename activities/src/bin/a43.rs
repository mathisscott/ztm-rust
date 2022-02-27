// Topic: Closures/Lambdas and Functions

#[derive(Clone)]
struct WithCall<F> {
    func: F,
}

impl<F> WithCall<F>
where
    F: Fn(i32, i32) -> i32,
{
    pub fn new(func: F) -> Self {
        WithCall { func }
    }

    pub fn run(&self, a: i32, b: i32) -> i32 {
        (self.func)(a, b)
    }
}

// fn math(a: i32, b: i32, op: Box<&dyn Fn(i32, i32) -> i32>) -> i32 {
//     // a closure's type needs to be a Box (a type of pointer)
//     // dyn keyword means that the data in the Fn() could be different things
//     op(a, b)
// }

// intent... make a way to reuse a closure over and over
fn main() {
    let adder = WithCall::new(|a, b| a + b);
    println!("{}", adder.run(2, 2));
    println!("{}", adder.run(4, 4));
    println!("{}", adder.run(10, 10));
}