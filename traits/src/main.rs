/**
 * Traits define similar functionality for different types.
 * Trait functions can be called on any type that implements the trait.
 * they are just regular functions that almost always take self as their first parameter.
 * using the impl keyword to implement a trait for a type.
 */

trait Noise {
    fn make_noise(&self);
}

struct Dog;
impl Noise for Dog {
    fn make_noise(&self) {
        println!("Bark");
    }
}

struct Cat;
impl Noise for Cat {
    fn make_noise(&self) {
        println!("Meow");
    }
}

fn hello(noisy: &impl Noise) {
    noisy.make_noise();
}

trait Shape {
    fn parameter(&self) -> f64;
}

struct Square {
    side: f64,
}
impl Shape for Square {
    fn parameter(&self) -> f64 {
        self.side * 4.0
    }
}

struct Triangle {
    sides: [f64; 3],
}
impl Shape for Triangle {
    fn parameter(&self) -> f64 {
        self.sides.iter().sum()
    }
}

fn print_parameter(shape: &impl Shape) {
    println!("Parameter: {}", shape.parameter());
}

struct Package {
    weight: f64,
    destination: String,
}
impl Package {
    fn new(weight: f64, destination: String) -> Self {
        Self {
            weight,
            destination,
        }
    }
}

impl Default for Package {
    fn default() -> Self {
        Self {
            weight: 1.0,
            destination: String::from("Earth"),
        }
    }
}

fn main() {
    let dog = Dog;
    let cat = Cat;

    hello(&dog);
    hello(&cat);

    let square = Square { side: 4.0 };
    let triangle = Triangle {
        sides: [3.0, 4.0, 5.0],
    };
    print_parameter(&square);
    print_parameter(&triangle);

    let package = Package::new(2.0, String::from("Mars"));
    let default_package = Package::default();
    println!("Package weight: {}", package.weight);
    println!("Package destination: {}", package.destination);
    println!("Default package weight: {}", default_package.weight);
    println!(
        "Default package destination: {}",
        default_package.destination
    );
}
