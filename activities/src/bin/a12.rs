// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics

struct Box {
    dimensions: Dimensions,
    weight: f64,
    color: BoxColor
}

struct Dimensions {
    width: f64,
    height: f64,
    depth: f64
}

enum BoxColor {
    Brown,
    White
}

impl Box {
    fn new(weight: f64, color: BoxColor, dimensions: Dimensions) -> Self {
        Self {
            dimensions,
            weight,
            color
        }
    }

    fn printout(&self) {
        let w = self.dimensions.width;
        let h = self.dimensions.height;
        let d = self.dimensions.depth;
        let color = match self.color {
            BoxColor::Brown => "brown",
            BoxColor::White => "white"
        };
        println!("I am a box which is {:?}\" wide, {:?}\" tall, and {:?}\" deep. I am {} and weigh {:?} pounds.", w, h, d, color, self.weight);
    }
}

fn main() {
    let my_box = Box::new(8.64, BoxColor::Brown, Dimensions { width: 11.4, height: 7.0, depth: 14.2});
    let my_other_box = Box {
        dimensions: Dimensions { width: 5.0, height: 3.48, depth: 7.25 },
        weight: 4.0,
        color: BoxColor::White
    };
    my_box.printout();
    my_other_box.printout();
}
