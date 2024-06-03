use std::collections::HashMap;

use crate::bill::Bill;

pub struct Database {
    pub bills: HashMap<String, Bill>,
}

impl Database {
    pub fn new() -> Self {
        Database {
            bills: HashMap::new(),
        }
    }
    pub fn add_bill(&mut self, name: String, amount: f64) {
        let bill = Bill::new(name, amount);
        self.bills.insert(bill.id.clone(), bill);
    }
    pub fn view_bills(&self) {
        if self.bills.is_empty() {
            println!("No bills found");
            return;
        }
        for (id, bill) in &self.bills {
            println!("ID: {}, Name: {}, Amount: {}", id, bill.name, bill.amount);
        }
    }
    pub fn remove_bill(&mut self, id: &str) {
        self.bills.remove(id);
    }
    pub fn edit_bill(&mut self, id: &str, amount: Option<f64>, name: Option<String>) {
        if let Some(bill) = self.bills.get_mut(id) {
            bill.amount = match amount {
                Some(val) => val,
                None => bill.amount,
            };
            bill.name = match name {
                Some(val) => val,
                None => bill.name.clone(),
            };
        }
    }
    pub fn get_bill(&self, id: &str) -> Option<&Bill> {
        self.bills.get(id)
    }
}
