// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor

enum Flavor {
    Tangy,
    Sweet
}

struct Drink {
    flavor: Flavor,
    ounces: f64,
    name: &'static str
}

fn drink_facts(drink: Drink) {
    let flavor: &'static str;
    match drink.flavor {
        Flavor::Tangy => flavor = "tangy",
        Flavor::Sweet => flavor = "sweet"
    };
    println!("A {:?} is a drink that is {:?} ounces and tastes {:?}", drink.name, drink.ounces, flavor);
}

fn main() {
    let margarita = Drink {
        name: "margarita",
        flavor: Flavor::Tangy,
        ounces: 4.0
    };
    let hi_c = Drink {
        name: "Hi-C",
        flavor: Flavor::Sweet,
        ounces: 7.4
    };
    drink_facts(margarita);
    drink_facts(hi_c);
}
