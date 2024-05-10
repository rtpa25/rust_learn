fn main() {
    let mut counter = 0;
    loop {
        if counter == 5 {
            break;
        }
        println!("Hello, world!, {:?}", counter);
        counter += 1;
    }
    println!("Hello, world!, {:?}", counter);

    let mut counter = 0;
    while counter < 5 {
        println!("Hello, world!, {:?}", counter);
        counter += 1;
    }

    let mut val = 5;
    while val > 0 {
        println!("ppp, {:?}", val);
        val -= 1;
    }
    println!("ppp, {:?}", val);
}
