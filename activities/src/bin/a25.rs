// Topic: Traits
//
// Requirements:
// * Calculate the perimeter of a square and triangle:
//   * The perimeter of a square is the length of any side*4.
//   * The perimeter of a triangle is a+b+c where each variable
//     represents the length of a side.
// * Print out the perimeter of the shapes
//
// Notes:
// * Use a trait to declare a perimeter calculation function
// * Use a single function to print out the perimeter of the shapes
//   * The function must utilize impl trait as a function parameter

trait ShapeCalculation {
    fn perimeter(&self) -> u64;
}

struct Square {
    side: u64,
}
impl ShapeCalculation for Square {
    fn perimeter(&self) -> u64 {
        4 * self.side
    }
}

struct Triangle {
    sides: Vec<u64>,
}
impl ShapeCalculation for Triangle {
    fn perimeter(&self) -> u64 {
        let mut perim = 0;
        for s in &self.sides {
            perim = perim + s;
        }
        perim
    }
}

fn main() {
    let square_perimeter = (Square { side: 2 }).perimeter();
    let triangle_perimeter = (Triangle { sides: vec![1, 2, 3]}).perimeter();

    println!("square = {square_perimeter:?}");
    println!("triangle = {triangle_perimeter:?}");
}
