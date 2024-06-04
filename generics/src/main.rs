/**
 * Generic functions are type of functions that can have parameters of various types
 * Traits are used as parmater types instead of actual types
 */

// trait Trait1 {}
// trait Trait2 {}
// trait Trait3 {}

// fn fn1(param1: impl Trait1, param2: impl Trait2) {}

// fn fn2<T: Trait1, U: Trait2>(param1: T, param2: U) {}

// fn fn3<T, U>(param1: T, param2: U)
// where
//     T: Trait1 + Trait3,
//     U: Trait2 + Trait3,
// {
// }

trait CheckIn {
    fn check_in(&self);
    fn process(&self);
}

struct Pilot;
impl CheckIn for Pilot {
    fn check_in(&self) {
        println!("Pilot check in");
    }
    fn process(&self) {
        println!("Pilot process");
    }
}
struct Passenger;
impl CheckIn for Passenger {
    fn check_in(&self) {
        println!("Passenger check in");
    }
    fn process(&self) {
        println!("Passenger process");
    }
}
struct Baggage;
impl CheckIn for Baggage {
    fn check_in(&self) {
        println!("Baggage check in");
    }
    fn process(&self) {
        println!("Baggage process");
    }
}

fn process_item<T: CheckIn>(item: &T) {
    item.check_in();
    item.process();
}

/**
 * Generic strcutures store data of any type within a structure
 * Trait bounds restrict the type of data the strcuture can utilize
 * also known as generic constraints
 * Usefull when making own data collections
 */
/**
 * can not mix generic structures in a single collection like a vector
 * generic strcutures expand to strcutures of specific type
 * generic strcutures hav two different syntaxes as shown below
 */
// trait Trait1 {}
// trait Trait2 {}
// trait Trait3 {}
// struct Name<T: Trait1 + Trait2, U: Trait3> {
//     field1: T,
//     field2: U,
// }

// struct Name2<T, U>
// where
//     T: Trait1 + Trait2,
//     U: Trait3,
// {
//     field1: T,
//     field2: U,
// }
// impl<T,U> Name<T,U>
// where T: Trait1 + Trait2,
//       U: Trait3
// {
//     fn new(field1: T, field2: U) -> Self {
//         Name { field1, field2 }
//     }
// }

struct Dimensions {
    width: f32,
    height: f32,
    depth: f32,
}

trait Convey {
    fn weight(&self) -> f32;
    fn dimensions(&self) -> Dimensions;
}

struct ConveyorBelt<T: Convey> {
    pub items: Vec<T>,
}

impl<T: Convey> ConveyorBelt<T> {
    pub fn add(&mut self, item: T) {
        self.items.push(item);
    }
}

struct CarPart {
    weight: f32,
    height: f32,
    depth: f32,
    width: f32,
    part_id: String,
}

impl Default for CarPart {
    fn default() -> Self {
        Self {
            weight: 0.0,
            height: 0.0,
            depth: 0.0,
            width: 0.0,
            part_id: String::from(""),
        }
    }
}

impl Convey for CarPart {
    fn weight(&self) -> f32 {
        self.weight
    }
    fn dimensions(&self) -> Dimensions {
        Dimensions {
            width: self.width,
            height: self.height,
            depth: self.depth,
        }
    }
}

trait Body {
    fn body(&self) -> String;
}
trait Color {
    fn color(&self) -> String;
}

struct Veichle<B: Body, C: Color> {
    body: B,
    color: C,
}

impl<B: Body, C: Color> Veichle<B, C> {
    pub fn new(body: B, color: C) -> Self {
        Self { body, color }
    }
}

struct Car;
impl Body for Car {
    fn body(&self) -> String {
        String::from("Car body")
    }
}

struct Truck;
impl Body for Truck {
    fn body(&self) -> String {
        String::from("Truck body")
    }
}

struct Red;
impl Color for Red {
    fn color(&self) -> String {
        String::from("Red color")
    }
}

struct Blue;
impl Color for Blue {
    fn color(&self) -> String {
        String::from("Blue color")
    }
}

fn main() {
    println!("Hello, world!");
    let pilot = Pilot;
    let passenger = Passenger;
    let baggage = Baggage;

    process_item(&pilot);
    process_item(&passenger);
    process_item(&baggage);

    let mut belt = ConveyorBelt { items: Vec::new() };
    let part = CarPart::default();
    belt.add(part);

    let red_truck = Veichle::new(Truck, Red);
    let blue_car = Veichle::new(Car, Blue);

    println!(
        "Red truck: {}, {}",
        red_truck.body.body(),
        red_truck.color.color()
    );
    println!(
        "Blue car: {}, {}",
        blue_car.body.body(),
        blue_car.color.color()
    );
}
