// Topic: New type pattern
//
// Requirements:
// * Display the selected color of shoes, a shirt, and pants
// * Create and display at least one of each type of clothes and color
//
// Notes:
// * Create a new type for each clothing item that wraps the Color enum
//   * Each new type should implement a `new` function
// * Create a function for each type of clothes (shoes, shirt, pants)
//   that accepts the new type specific to that type of clothing

#[derive(Debug)]
enum Color {
    Black,
    Blue,
    Brown,
    Custom(String),
    Gray,
    Green,
    Purple,
    Red,
    White,
    Yellow,
}

#[derive(Debug)]
struct ShoeColor(Color);
impl ShoeColor {
    fn new(c: Color) -> Self {
        Self(c)
    }
}

#[derive(Debug)]
struct ShirtColor(Color);
impl ShirtColor {
    fn new(c: Color) -> Self {
        Self(c)
    }
}

#[derive(Debug)]
struct PantsColor(Color);
impl PantsColor {
    fn new(c: Color) -> Self {
        Self(c)
    }
}

fn print_shirt_color(color: ShirtColor) {
    println!("shirt color = {color:?}");
}

fn print_shoe_color(color: ShoeColor) {
    println!("shoe color = {color:?}");
}

fn print_pants_color(color: PantsColor) {
    println!("pants color = {color:?}");
}

fn main() {
    let pants = PantsColor::new(Color::Custom("charcoal".to_owned()));
    let shoes = ShoeColor::new(Color::Black);
    let shirt = ShirtColor::new(Color::White);

    print_pants_color(pants);
    print_shirt_color(shirt);
    print_shoe_color(shoes);
}
