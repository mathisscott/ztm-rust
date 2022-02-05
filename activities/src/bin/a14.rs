// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function

enum Color {
    Red,
    Blue,
    Purple
}

impl Color {
    fn to_string(&self) -> &str {
        let return_str = match self {
            Color::Red => "red",
            Color::Blue => "blue",
            Color::Purple => "purple"
        };
        return_str
    }
}

struct Person {
    name: String,
    age: u32,
    favorite_color: Color
}

impl Person {
    fn new(name: String, age: u32, favorite_color: Color) -> Self {
        Self {
            name,
            age,
            favorite_color
        }
    }

    fn print(&self) {
        if self.age > 10 {
            println!("My name is {} and I am {:?} years old", self.name, self.age);
        } else {
            println!("My name is {}. I am {:?} years old and my favorite color is {}.", self.name, self.age, self.favorite_color.to_string());
        }
    }
}

fn main() {
    let people = vec![
        Person::new("Todd".to_owned(), 32, Color::Purple),
        Person::new(String::from("Bobby"), 8, Color::Red),
        Person::new("Beatrice".to_owned(), 6, Color::Blue),
    ];

    for p in people {
        p.print();
    }
}
