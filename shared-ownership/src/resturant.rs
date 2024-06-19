use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
pub enum MenuItem {
    Drink,
    Salad,
}

#[derive(Debug)]
pub struct ItemOrder {
    item: MenuItem,
    quantity: u32,
}

#[derive(Debug)]
pub struct TableOrder {
    items: Vec<ItemOrder>,
}

pub fn new_table_order() -> TableOrder {
    TableOrder {
        items: vec![
            ItemOrder {
                item: MenuItem::Drink,
                quantity: 2,
            },
            ItemOrder {
                item: MenuItem::Salad,
                quantity: 1,
            },
        ],
    }
}

pub type Order = Rc<RefCell<Vec<TableOrder>>>;

#[derive(Debug)]
pub struct Chef(pub Order);

#[derive(Debug)]
pub struct WaitStaff(pub Order);

#[derive(Debug)]
pub struct Accounting(pub Order);
