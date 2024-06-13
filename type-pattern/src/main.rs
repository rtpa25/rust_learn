pub mod never_zero;

fn divide(a: i32, b: never_zero::NeverZero) -> i32 {
    a / b.0
}

// Topic: New type pattern
//
// Requirements:
// * Display the selected color of shoes, a shirt, and pants
// * Create and display at least one of each type of clothes and color
//
// Notes:
// * Create a new type for each clothing item that wraps the Color enum
//   * Each new type should implement a `new` function
// * Create a function for each type of clothes (shoes, shirt, pants)
//   that accepts the new type specific to that type of clothing

enum Color {
    Black,
    Blue,
    Brown,
    Custom(String),
    Gray,
    Green,
    Purple,
    Red,
    White,
    Yellow,
}

struct Shoes(Color);
impl Shoes {
    fn new(color: Color) -> Self {
        Self(color)
    }
}
struct Shirt(Color);
impl Shirt {
    fn new(color: Color) -> Self {
        Self(color)
    }
}
struct Pants(Color);
impl Pants {
    fn new(color: Color) -> Self {
        Self(color)
    }
}

fn check_shoe(shoe: Shoes) {
    match shoe.0 {
        Color::Black => println!("Black shoes"),
        Color::Blue => println!("Blue shoes"),
        Color::Brown => println!("Brown shoes"),
        Color::Custom(color) => println!("Custom color shoes: {}", color),
        Color::Gray => println!("Gray shoes"),
        Color::Green => println!("Green shoes"),
        Color::Purple => println!("Purple shoes"),
        Color::Red => println!("Red shoes"),
        Color::White => println!("White shoes"),
        Color::Yellow => println!("Yellow shoes"),
    }
}

fn check_shirt(shirt: Shirt) {
    match shirt.0 {
        Color::Black => println!("Black shirt"),
        Color::Blue => println!("Blue shirt"),
        Color::Brown => println!("Brown shirt"),
        Color::Custom(color) => println!("Custom color shirt: {}", color),
        Color::Gray => println!("Gray shirt"),
        Color::Green => println!("Green shirt"),
        Color::Purple => println!("Purple shirt"),
        Color::Red => println!("Red shirt"),
        Color::White => println!("White shirt"),
        Color::Yellow => println!("Yellow shirt"),
    }
}

fn check_pants(pants: Pants) {
    match pants.0 {
        Color::Black => println!("Black pants"),
        Color::Blue => println!("Blue pants"),
        Color::Brown => println!("Brown pants"),
        Color::Custom(color) => println!("Custom color pants: {}", color),
        Color::Gray => println!("Gray pants"),
        Color::Green => println!("Green pants"),
        Color::Purple => println!("Purple pants"),
        Color::Red => println!("Red pants"),
        Color::White => println!("White pants"),
        Color::Yellow => println!("Yellow pants"),
    }
}

fn main() {
    let a = 10;
    let b = match never_zero::NeverZero::new(2) {
        Ok(value) => value,
        Err(message) => {
            println!("Error: {}", message);
            return;
        }
    };
    let result = divide(a, b);
    println!("Result: {}", result);

    let black_shoes = Shoes::new(Color::Black);
    check_shoe(black_shoes);

    let blue_shirt = Shirt::new(Color::Blue);
    check_shirt(blue_shirt);

    let brown_pants = Pants::new(Color::Brown);
    check_pants(brown_pants);
}
