use crate::{db, util};

pub struct Manager {
    pub db: db::Database,
}

impl Manager {
    pub fn new(db: db::Database) -> Self {
        Manager { db }
    }

    pub fn add_bill(&mut self) {
        println!("Enter bill name: ");
        let name = match util::collect_user_input() {
            Ok(val) => val,
            Err(e) => panic!("Error occured while reading user data: {:?}", e),
        };
        println!("Enter bill amount: ");
        let amount = match util::collect_user_input() {
            Ok(val) => match val.parse::<f64>() {
                Ok(val) => val,
                Err(e) => panic!("Error occured while parsing amount: {:?}", e),
            },
            Err(e) => panic!("Error occured while reading user data: {:?}", e),
        };

        self.db.add_bill(name, amount);
        println!("Bill added successfully")
    }

    pub fn view_bills(&self) {
        self.db.view_bills();
    }

    pub fn remove_bill(&mut self) {
        self.db.view_bills();
        println!("Enter bill id you wanna remove: ");
        let id = match util::collect_user_input() {
            Ok(val) => val,
            Err(e) => panic!("Error occured while reading user data: {:?}", e),
        };
        self.db.remove_bill(&id);
        println!("Bill removed successfully");
    }

    pub fn edit_bill(&mut self) -> Result<(), String> {
        self.db.view_bills();
        println!("Enter bill id you wanna edit: ");
        let id = match util::collect_user_input() {
            Ok(val) => val,
            Err(e) => panic!("Error occured while reading user data: {:?}", e),
        };
        let bill = self.db.get_bill(&id);
        if bill.is_none() {
            return Err("Bill not found".to_string());
        }

        println!("Enter new amount, for bill id: {}", id);
        println!("hit enter to keep the same amount");
        let mut amount = match util::collect_user_input() {
            Ok(val) => match val.parse::<f64>() {
                Ok(val) => Some(val),
                Err(_) => None,
            },
            Err(_) => None,
        };
        if amount.is_none() {
            amount = Some(bill.unwrap().amount);
        }

        println!("Enter new name, for bill id: {}", id);
        println!("hit enter without anything to keep the same name");
        let mut name = match util::collect_user_input() {
            Ok(val) => {
                if val.is_empty() {
                    None
                } else {
                    Some(val)
                }
            }
            Err(_) => None,
        };
        if name.is_none() {
            name = Some(bill.unwrap().name.clone());
        }

        if name == Some(bill.unwrap().name.clone()) && amount == Some(bill.unwrap().amount) {
            return Err("No changes made".to_string());
        }

        println!(
            "Bill with id: {} will be updated to amount: {:?} and name: {:?}",
            id, amount, name
        );
        println!("Are you sure you want to update? (y/n)");
        let confirm = match util::collect_user_input() {
            Ok(val) => val,
            Err(e) => panic!("Error occured while reading user data: {:?}", e),
        };
        if confirm != "y" {
            return Err("Exiting...".to_string());
        }
        self.db.edit_bill(&id, amount, name);
        println!("Bill updated successfully");
        Ok(())
    }
}
