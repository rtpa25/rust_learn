struct GrocerryItem {
    id: i32,
    quantity: i32,
}

fn print_quantity(grocerry_item: &GrocerryItem) {
    println!("Quantity: {}", grocerry_item.quantity)
}

fn print_id(grocerry_item: &GrocerryItem) {
    println!("ID: {}", grocerry_item.id)
}

struct ShippingBox {
    dimensions: (f64, f64, f64),
    weight: f64,
    color: Color,
}

#[derive(Debug)]
enum Color {
    RED,
    GREEN,
    BLUE,
    YELLOW,
}

impl ShippingBox {
    fn contruct(dimensions: (f64, f64, f64), weight: f64, color: Color) -> Self {
        Self {
            color,
            dimensions,
            weight,
        }
    }

    fn print_properties(&self) {
        println!("Dimensions: {:?}", self.dimensions);
        println!("Weight: {}", self.weight);
        println!("Color: {:?}", self.color);
    }
}

fn main() {
    let item = GrocerryItem { id: 1, quantity: 2 };

    print_id(&item);
    print_quantity(&item);

    let red_shipping_box = ShippingBox::contruct((10.0, 10.0, 10.0), 5.0, Color::RED);
    let green_shipping_box = ShippingBox::contruct((10.0, 10.0, 10.0), 5.0, Color::GREEN);
    let blue_shipping_box = ShippingBox::contruct((10.0, 10.0, 10.0), 5.0, Color::BLUE);
    let yellow_shipping_box = ShippingBox::contruct((10.0, 10.0, 10.0), 5.0, Color::YELLOW);

    red_shipping_box.print_properties();
    green_shipping_box.print_properties();
    blue_shipping_box.print_properties();
    yellow_shipping_box.print_properties();
}
