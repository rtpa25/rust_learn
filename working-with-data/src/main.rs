enum Direction {
    North,
    East,
    South,
    West,
}

struct ShippingBox {
    depth: u32,
    width: u32,
    height: u32,
}

impl ShippingBox {
    fn volume(&self) -> u32 {
        self.depth * self.width * self.height
    }
}

struct GrocerryItem {
    stock: u32,
    price: f32,
}

impl GrocerryItem {
    fn total_price(&self) -> f32 {
        self.stock as f32 * self.price
    }
}

enum Flavours {
    Vanilla,
    Chocolate,
    Strawberry,
}

struct IceCream {
    flavour: Flavours,
    size: u32,
}

fn main() {
    let direction = Direction::East;

    match direction {
        Direction::North => println!("Go North!"),
        Direction::East => println!("Go East!"),
        Direction::South => println!("Go South!"),
        Direction::West => println!("Go West!"),
    }

    enum Color {
        Red,
        Green,
        Blue,
        RgbColor(u8, u8, u8),
    }

    let color = Color::RgbColor(0, 160, 255);

    let shipping_box = ShippingBox {
        depth: 10,
        width: 20,
        height: 30,
    };

    println!(
        "The volume of the shipping box is: {}",
        shipping_box.volume()
    );

    let grocerry_item = GrocerryItem {
        stock: 100,
        price: 1.99,
    };

    println!("The total stock of items is: {}", grocerry_item.stock);
    println!("The price of each item is: {}", grocerry_item.price);

    println!(
        "The total price of the grocerry items is: {}",
        grocerry_item.total_price()
    );

    let mut ice_cream = IceCream {
        flavour: Flavours::Vanilla,
        size: 10,
    };

    print_ice_cream(&ice_cream);

    ice_cream.flavour = Flavours::Chocolate;

    print_ice_cream(&ice_cream);

    let coordinate = (10, 20);
    println!("The coordinate is: {:?}", coordinate);

    let (x, y) = coordinate;
    println!("The x coordinate is: {}", x);
    println!("The y coordinate is: {}", y);

    if y > 5 {
        println!("The y coordinate is greater than 5");
    } else if y < 5 {
        println!("The y coordinate is less than 5");
    } else {
        println!("The y coordinate is equal to 5");
    }

    let coorinate = Coordinate(10, 20);
    println!("The coordinate is: {:?}", coorinate);

    let Coordinate(x1, y1) = coorinate;
    println!("The x coordinate is: {}", x1);
    println!("The y coordinate is: {}", y1);
}

#[derive(Debug)] // This is used to print the struct using {:?}
struct Coordinate(i32, i32); // Tuple struct

fn print_ice_cream(ice_cream: &IceCream) {
    println!("The ice cream is of size: {}", ice_cream.size);
    match ice_cream.flavour {
        Flavours::Vanilla => println!("The ice cream is Vanilla flavoured"),
        Flavours::Chocolate => println!("The ice cream is Chocolate flavoured"),
        Flavours::Strawberry => println!("The ice cream is Strawberry flavoured"),
    }
}

fn expressions() {
    let my_num = 3;
    let is_lt_5 = if my_num < 5 { true } else { false };
    let message = match my_num {
        1 => "One",
        2 => "Two",
        _ => "Other",
    };

    enum Menu {
        Burger,
        Fries,
        Drink,
    }

    let paid = true;
    let item = Menu::Burger;
    let burger_price = 5.99;

    let price = match item {
        Menu::Burger => {
            if paid {
                burger_price
            } else {
                0.0
            }
        }
        Menu::Fries => 2.99,
        Menu::Drink => 1.99,
    };
}

enum Access {
    Admin,
    Manager,
    User,
    Guest,
}

fn authorisation() {
    let access_level = Access::Admin;
    let can_access_file = match access_level {
        Access::Admin => true,
        Access::Manager => true,
        Access::User => false,
        Access::Guest => false,
    };

    if can_access_file {
        println!("You can access the file");
    } else {
        println!("You cannot access the file");
    }
}

fn print_big_small(num: i32) {
    let is_big = num > 100;

    match is_big {
        true => println!("The number is big"),
        false => println!("The number is small"),
    }
}
