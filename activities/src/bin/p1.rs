// Project 1: Interactive bill manager
//
// Summary:
//   Create a command line bills/expenses manager that runs
//   interactively. This mini project brings together many of
//   the concepts learn thus far into a single application.
//
//   The user stories/requirements are split into stages.
//   Fully implement each stage as a complete working program
//   before making changes for the next stage. Leverage the
//   compiler by using `cargo check --bin p1` when changing
//   between stages to help identify adjustments that need
//   to be made.
//
// User stories:
// * Stage 1:
//   - I want to add bills, including the name and amount owed.
//   - I want to view existing bills.
// * Stage 2:
//   - I want to remove bills.
// * Stage 3:
//   - I want to edit existing bills.
//   - I want to go back if I change my mind.
//
// Tips:
// * Use the loop keyword to create an interactive menu.
// * Each menu choice should be it's own function, so you can work on the
//   the functionality for that menu in isolation.
// * A vector is the easiest way to store the bills at stage 1, but a
//   hashmap will be easier to work with at stages 2 and 3.

use std::collections::HashMap;
use std::io;

#[derive(Debug, Clone)]
pub struct Bill {
    name: String,
    amount: f64,
}

pub struct Bills {
    inner: HashMap<String, Bill>
}

impl Bills {
    fn new() -> Self {
        Self {
            inner: HashMap::new()
        }
    }

    fn add(&mut self, bill: Bill) {
        self.inner.insert(bill.name.to_string(), bill);
    }

    fn all(&self) -> Vec<&Bill> {
        self.inner.values().collect()
    }

    fn delete(&mut self, name: &str) -> bool {
        self.inner.remove(name).is_some()
    }

    fn update(&mut self, name: &str, amount: f64) -> bool {
        match self.inner.get_mut(name) {
            Some(bill) => {
                bill.amount = amount;
                true
            },
            None => false
        }
    }
}

enum BillingMenu {
    Add,
    View,
    Remove,
    Update
}

impl BillingMenu {
    fn from_str(input: &str) -> Option<BillingMenu> {
        match input {
            "1" => Some(Self::Add),
            "2" => Some(Self::View),
            "3" => Some(Self::Remove),
            "4" => Some(Self::Update),
            _ => None,
        }
    }

    fn show_intro() {
        let intro = vec![
            "",
            " == Bill Manager == ",
            "1. Add Bill",
            "2. View Bills",
            "3. Remove Bill",
            "4. Update Bill Amount",
            "x. Exit",
            "",
            "Enter option: ",
        ];

        for ln in intro {
            println!("{ln}");
        }
    }
}

fn get_user_input() -> Option<String> {
    let mut buffer = String::new();
    while io::stdin().read_line(&mut buffer).is_err() {
        println!("Please re-enter data...");
    }
    let uinput = buffer.trim().to_owned();
    if &uinput == "" {
        None
    } else {
        Some(uinput)
    }
}

fn get_bill_amount() -> Option<f64> {
    println!("Enter amount: ");
    loop {
        let input = match get_user_input() {
            Some(n) => n,
            None => return None,
        };

        if &input == "" {
            return None;
        }

        let parsed_input: Result<f64, _> = input.parse();
        match parsed_input {
            Ok(amt) => return Some(amt),
            Err(_) => println!("Please enter your amount: "),
        }

    }
}

mod menu {
    use crate::{ Bills, Bill, get_user_input, get_bill_amount };

    pub fn add_bill(bills: &mut Bills) {
        println!("Enter bill name:");
        let name = match get_user_input() {
            Some(n) => n,
            None => return,
        };
        let amount = match get_bill_amount() {
            Some(a) => a,
            None => return,
        };
        let bill = Bill { name, amount };
        bills.add(bill);
        println!("Bill was added!");
    }

    pub fn view_bills(bills: &Bills) {
        for b in bills.all() {
            println!("{b:?}");
        }
    }

    pub fn remove_bill(bills: &mut Bills) {
        for b in bills.all() {
            println!("{b:?}");
        }
        println!("Remove which one?");
        let name = match get_user_input() {
            Some(n) => n,
            None => return,
        };
        if bills.delete(&name) {
            println!("Success! Bill removed.");
        } else {
            println!("Bill not found.");
        }
    }

    pub fn update_bill(bills: &mut Bills) {
        for b in bills.all() {
            println!("{b:?}");
        }
        println!("Enter bill to update");
        let name = match get_user_input() {
            Some(n) => n,
            None => return,
        };
        let amount = match get_bill_amount() {
            Some(n) => n,
            None => return,
        };
        if bills.update(&name, amount) {
            println!("Success! Bill updated.");
        } else {
            println!("Bill not found.");
        }
    }
}

fn run_pgm() -> Option<()> {
    let mut bills = Bills::new();

    loop {
        BillingMenu::show_intro();
        let input = get_user_input()?;
        match BillingMenu::from_str(input.as_str()) {
            Some(BillingMenu::Add) => menu::add_bill(&mut bills),
            Some(BillingMenu::View) => menu::view_bills(&bills),
            Some(BillingMenu::Remove) => menu::remove_bill(&mut bills),
            Some(BillingMenu::Update) => menu::update_bill(&mut bills),
            None => break,
        }
    }
    None
}

fn main() {
    run_pgm();
}