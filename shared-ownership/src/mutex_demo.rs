use std::sync::Arc;

use parking_lot::Mutex;

pub type SharedSignData = Arc<Mutex<String>>;

pub struct DigitalSignBoard {
    display: SharedSignData,
}

impl DigitalSignBoard {
    pub fn new(data: SharedSignData) -> Self {
        DigitalSignBoard { display: data }
    }

    pub fn update(&self, message: &str) {
        let mut data = self.display.lock();
        *data = message.to_string();
    }

    pub fn display(&self) -> String {
        let data = self.display.lock();
        data.clone()
    }
}

pub fn spawn_display_thread(data: SharedSignData) {
    std::thread::spawn(move || {
        let sign = DigitalSignBoard::new(data);
        loop {
            sign.update("Welcome to Rustville!");
            std::thread::sleep(std::time::Duration::from_millis(200));
            sign.update("Please wear a mask!");
            std::thread::sleep(std::time::Duration::from_millis(200));
            sign.display();
        }
    });
}
