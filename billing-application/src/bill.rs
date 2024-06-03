use nanoid::nanoid;

pub struct Bill {
    pub id: String,
    pub name: String,
    pub amount: f64,
}

impl Bill {
    pub fn new(name: String, amount: f64) -> Bill {
        Bill {
            id: nanoid!(),
            name,
            amount,
        }
    }
}
