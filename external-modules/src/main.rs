use garden::vegetable::Asparagus;

pub mod garden;

fn main() {
    let a = Asparagus { stalks: 0 };
    println!("Hello, world!, {:?}", a);
}
