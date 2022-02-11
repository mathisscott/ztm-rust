// Topic: Result
//
// Requirements:
// * Determine if a customer is able to make a restricted purchase
// * Restricted purchases require that the age of the customer
//   is at least 21
//
// Notes:
// * Use a struct to store at least the age of a customer
// * Use a function to determine if a customer can make a restricted purchase
// * Return a result from the function
// * The Err variant should detail the reason why they cannot make a purchase

struct Customer {
    age: u32
}

fn can_purchase(item_is_restricted: bool, cust: &Customer) -> Result<(), String> {
    match item_is_restricted {
        true => {
            if cust.age < 21 {
                return Err("not old enough to buy".to_owned());
            } else {
                return Ok(());
            }
        },
        false => { return Ok(()); },
    }
}

fn main() {
    let old_cust = Customer { age: 30 };
    let young_cust = Customer { age: 18 };

    let items = vec![can_purchase(true, &young_cust), can_purchase(false, &young_cust), can_purchase(true, &old_cust)];

    for i in items {
        match i {
            Err(str) => println!("{:?}", str),
            _ => println!("ok to buy"),
        }
    }
}
