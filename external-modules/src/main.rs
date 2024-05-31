pub mod garden;
pub mod plant;

fn main() {
    let a = garden::Asparagus { stalks: 0 };
    println!("Hello, world!, {:?}", a);
    plant::grow();
}
