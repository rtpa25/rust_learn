/**
 * Stack
 * Sequesntial memory allocation
 * used for local variables
 * fast access
 * limited size
 * must know the size at compile time
 *
 * Heap
 * Algorythmic memory allocation
 * used for large amounts of data
 * unlimited size
 * slower access
 * dynmically sized data/unknown sized data
 */

/**
 * Trait object basics
 * "Run time generics"
 * - More flexible than generics
 * Allows mixed types in a collection
 * easier to work with similar data types
 * polymorphic program behaviour at runtime
 * easily add new behaviours just by creating a new struct
 * small perf penalty
 */

/**
 * Trait object allow for composite collection
 * slightly less performant than generics
 * use the dyn keyword to create a trait object
 * trait objects can be borrowed using a ref or moved using a box
 * usually want to use a box when storing trait object in a vector
 */

struct Entry {
    id: i32,
}

trait Clicky {
    fn click(&self);
}

struct Keyboard;

impl Clicky for Keyboard {
    fn click(&self) {
        println!("Keyboard clicked");
    }
}

fn main() {
    let entry = Entry { id: 1 };
    println!("Hello, world! {}", entry.id);

    let entry_ptr: Box<Entry> = Box::new(entry);
    println!("Hello, world! {}", entry_ptr.id);

    let entry_stack = *entry_ptr;
    println!("Hello, world! {}", entry_stack.id);

    let keyboard = Keyboard;
    keyboard.click();

    let keeb_obj: &dyn Clicky = &keyboard;
    borrow_clicky(keeb_obj);

    let keeb_obj: &dyn Clicky = &Keyboard; // not recommended
    borrow_clicky(keeb_obj);

    let keeb_obj: Box<dyn Clicky> = Box::new(Keyboard);
    borrow_clicky(&*keeb_obj);
    move_clicky(keeb_obj);

    borrow_clicky(&keyboard);

    // Heteregenous vectors impl1
    let keeb: Box<dyn Clicky> = Box::new(Keyboard);
    let mouse: Box<dyn Clicky> = Box::new(Mouse);
    let clickers = vec![keeb, mouse];
    for clicker in clickers {
        clicker.click();
    }

    // Heteregenous vectors impl2
    let keeb = Box::new(Keyboard);
    let mouse = Box::new(Mouse);
    let clickers: Vec<Box<dyn Clicky>> = vec![keeb, mouse];
    for clicker in clickers {
        clicker.click();
    }

    let price = 20.0;
    let sales: Vec<Box<dyn Sale>> = vec![
        Box::new(FullSale(price)),
        Box::new(OneDollarOffCoupon(price)),
        Box::new(TenPercentOffPromo(price)),
    ];
    println!("Total sales: {}", total_sales(sales));

    let carpet = Box::new(Carpet { length: 10.0 });
    let tile = Box::new(Tile { length: 10.0 });
    let wood = Box::new(Wood { length: 10.0 });
    let materials: Vec<Box<dyn Material>> = vec![carpet, tile, wood];
    println!("Total cost: {}", total_cost(materials));
}

fn borrow_clicky(obj: &dyn Clicky) {
    obj.click();
}

fn move_clicky(obj: Box<dyn Clicky>) {
    obj.click();
}

struct Mouse;
impl Clicky for Mouse {
    fn click(&self) {
        println!("Mouse clicked");
    }
}

trait Sale {
    fn amount(&self) -> f64;
}

struct FullSale(f64);
impl Sale for FullSale {
    fn amount(&self) -> f64 {
        self.0
    }
}

struct OneDollarOffCoupon(f64);
impl Sale for OneDollarOffCoupon {
    fn amount(&self) -> f64 {
        self.0 - 1.0
    }
}

struct TenPercentOffPromo(f64);
impl Sale for TenPercentOffPromo {
    fn amount(&self) -> f64 {
        self.0 * 0.9
    }
}

fn total_sales(sales: Vec<Box<dyn Sale>>) -> f64 {
    sales.iter().map(|sale| sale.amount()).sum()
}

trait Material {
    fn const_per_meter(&self) -> f64;
    fn length(&self) -> f64;
    fn total_cost(&self) -> f64 {
        self.const_per_meter() * self.length()
    }
}

struct Carpet {
    length: f64,
}
impl Material for Carpet {
    fn const_per_meter(&self) -> f64 {
        10.0
    }
    fn length(&self) -> f64 {
        self.length
    }
    fn total_cost(&self) -> f64 {
        self.const_per_meter() * self.length()
    }
}

struct Tile {
    length: f64,
}
impl Material for Tile {
    fn const_per_meter(&self) -> f64 {
        20.0
    }
    fn length(&self) -> f64 {
        self.length
    }
    fn total_cost(&self) -> f64 {
        self.const_per_meter() * self.length()
    }
}

struct Wood {
    length: f64,
}
impl Material for Wood {
    fn const_per_meter(&self) -> f64 {
        30.0
    }
    fn length(&self) -> f64 {
        self.length
    }
    fn total_cost(&self) -> f64 {
        self.const_per_meter() * self.length()
    }
}

fn total_cost(materials: Vec<Box<dyn Material>>) -> f64 {
    materials.iter().map(|material| material.total_cost()).sum()
}
