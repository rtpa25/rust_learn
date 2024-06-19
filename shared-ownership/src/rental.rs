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

use std::{cell::RefCell, rc::Rc};

#[derive(Debug, Clone)]
pub enum VeichleType {
    Car,
    Truck,
    Van,
}

#[derive(Debug, PartialEq, Clone)]
pub enum VeichleStatus {
    Available,
    Unavailable,
    Maintenance,
    Rented,
}

#[derive(Debug, Clone)]
pub struct Rental {
    veichle_type: VeichleType,
    vin: String,
    status: VeichleStatus,
}
impl Rental {
    pub fn new(veichle_type: VeichleType, vin: String) -> Self {
        Self {
            veichle_type,
            vin,
            status: VeichleStatus::Available,
        }
    }
}

type RentalList = Rc<RefCell<Vec<Rental>>>;

#[derive(Debug)]
pub struct Corporate(pub RentalList);

impl Corporate {
    pub fn get_rentals(&self) -> Vec<Rental> {
        self.0.borrow().clone()
    }

    pub fn add_rental(&self, veichle_type: VeichleType, vin: String) {
        let rental = Rental::new(veichle_type, vin);
        self.0.borrow_mut().push(rental);
    }

    pub fn take_for_maintenance(&self, vin: &str) {
        let mut rentals = self.0.borrow_mut();
        for rental in rentals.iter_mut() {
            if rental.vin == vin && rental.status == VeichleStatus::Available {
                rental.status = VeichleStatus::Maintenance;
            }
        }
    }
}

#[derive(Debug)]
pub struct StoreFront(pub RentalList);

impl StoreFront {
    pub fn rent_vehicle(&self, vin: &str) {
        let mut rentals = self.0.borrow_mut();
        for rental in rentals.iter_mut() {
            if rental.vin == vin && rental.status == VeichleStatus::Available {
                rental.status = VeichleStatus::Rented;
            }
        }
    }
}

pub fn create_rentals() -> RentalList {
    Rc::new(RefCell::new(vec![
        Rental {
            veichle_type: VeichleType::Car,
            vin: "1".to_owned(),
            status: VeichleStatus::Available,
        },
        Rental {
            veichle_type: VeichleType::Truck,
            vin: "2".to_owned(),
            status: VeichleStatus::Available,
        },
        Rental {
            veichle_type: VeichleType::Van,
            vin: "3".to_owned(),
            status: VeichleStatus::Available,
        },
    ]))
}
