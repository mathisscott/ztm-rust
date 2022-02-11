// Topic: User input
//
// Requirements:
// * Verify user input against pre-defined keywords
// * The keywords represent possible power options for a computer:
//   * Off
//   * Sleep
//   * Reboot
//   * Shutdown
//   * Hibernate
// * If the user enters one of the keywords, a message should be printed to
//   the console indicating which action will be taken
//   * Example: If the user types in "shutdown" a message should display such
//     as "shutting down"
// * If the keyword entered does not exist, an appropriate error message
//   should be displayed
//
// Notes:
// * Use an enum to store the possible power states
// * Use a function with a match expression to print out the power messages
//   * The function should accept the enum as an input
// * Use a match expression to convert the user input into the power state enum
// * The program should be case-insensitive (the user should be able to type
//   Reboot, reboot, REBOOT, etc.)
use std::io;

enum PowerState {
    Off,
    Sleep,
    Reboot,
    Shutdown,
    Hibernate,
}

impl PowerState {
    fn new(s: &str) -> Option<PowerState> {
        let state: String = s.trim().to_lowercase();
        match state.as_str() {
            "off" => Some(PowerState::Off),
            "sleep" => Some(PowerState::Sleep),
            "reboot" => Some(PowerState::Reboot),
            "shutdown" => Some(PowerState::Shutdown),
            "hibernate" => Some(PowerState::Hibernate),
            _ => None,
        }
    }
}

fn print_power_state(state: PowerState) {
    use PowerState::*;
    match state {
        Off => println!("powering off"),
        Sleep => println!("going to sleep"),
        Reboot => println!("rebooting"),
        Shutdown => println!("shutting down"),
        Hibernate => println!("preserving power")
    }
}

fn main() {
    let mut buffer = String::new();
    println!("Enter power state:");
    let io_result = io::stdin().read_line(&mut buffer);

    if io_result.is_ok() {
        match PowerState::new(&buffer) {
            Some(ps) => print_power_state(ps),
            None => println!("invalid power state"),
        }
    } else {
        println!("error reading input!");
    }
}
