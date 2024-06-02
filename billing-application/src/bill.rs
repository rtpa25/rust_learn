use nanoid::nanoid;
use std::collections::HashMap;

pub struct Bill {
    pub id: String,
    pub name: String,
    pub amount: f64,
}

pub struct Database {
    pub bills: HashMap<String, Bill>,
}

impl Database {
    pub fn new() -> Self {
        Database {
            bills: HashMap::new(),
        }
    }
    pub fn add_bill(&mut self, bill: Bill) {
        self.bills.insert(bill.id.clone(), bill);
    }
    pub fn view_bills(&self) {
        for (id, bill) in &self.bills {
            println!("ID: {}, Name: {}, Amount: {}", id, bill.name, bill.amount);
        }
    }
    pub fn remove_bill(&mut self, id: &str) {
        self.bills.remove(id);
    }
    pub fn edit_bill(&mut self, id: &str, amount: f64) {
        if let Some(bill) = self.bills.get_mut(id) {
            bill.amount = amount;
        }
    }
}

impl Bill {
    pub fn add(name: String, amount: f64) -> Bill {
        Bill {
            id: nanoid!(),
            name,
            amount,
        }
    }
}
