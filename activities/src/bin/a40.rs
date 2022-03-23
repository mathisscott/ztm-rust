// Topic: Smart Pointers & RefCell
//
// Summary:
//   A vehicle rental company wants to access the rentals available
//   at storefront locations. Create a program that provides access
//   to storefront rentals from the corporate headquarters.
//
// Requirements:
// * Corporate must be able to access the rentals at a storefront
// * Storefronts must be able to rent out vehicles
// * Rentals have the following attributes:
//   - Type of vehicle
//   - Vehicle Identification Number (VIN)
//   - Vehicle status:
//     * Available, Unavailable, Maintenance, Rented
//
// Notes:
// * Use Rc and RefCell to create shared mutable data structures
// * Create at least two rentals and ensure that Corporate and StoreFront
//   can both access the rental information
// * Test your program by changing the vehicle status from both a storefront
//   and from corporate

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum Vehicle {
    Car,
    Truck,
}

#[derive(Debug, Hash, PartialOrd, PartialEq)]
enum Status {
    Available,
    Unavailable,
    Maintenance,
    Rented,
}

#[derive(Debug)]
struct Rental {
    vehicle: Vehicle,
    vin: String,
    status: Status,
}

#[derive(Debug)]
struct Corporate(Rentals);

#[derive(Debug)]
struct StoreFront(Rentals);

// impl StoreFront;
// rent out vehicles

type Rentals = Rc<RefCell<Vec<Rental>>>;

fn main() {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn update_status() {
        let vehicles = vec![
            Rental {
                vehicle: Vehicle::Car,
                vin: "1cf456".to_owned(),
                status: Status::Available,
            },
            Rental {
                vehicle: Vehicle::Truck,
                vin: "2dg567".to_owned(),
                status: Status::Maintenance,
            },
        ];

        let vehicles = Rc::new(RefCell::new(vehicles));
        let corp = Corporate(Rc::clone(&vehicles)); // this is making copies of the POINTER
        let store = StoreFront(Rc::clone(&vehicles));

        {
            let mut rentals = store.0.borrow_mut();
            if let Some(car) = rentals.get_mut(0) {
                assert_eq!(car.status, Status::Available);
                car.status = Status::Rented;
            }
        }

        {
            let mut rentals = corp.0.borrow_mut();
            if let Some(car) = rentals.get_mut(0) {
                assert_eq!(car.status, Status::Rented);
                car.status = Status::Available;
            }
        }

        let rentals = store.0.borrow();
        if let Some(car) = rentals.get(0) {
            assert_eq!(car.status, Status::Available);
        }
}
}