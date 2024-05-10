fn main() {
    let age = 15;

    if age < 18 {
        println!("You are a minor");
    } else {
        println!("You are an adult");
    }

    if age >= 21 {
        println!("You can drink alcohol");
    } else {
        println!("You can't drink alcohol");
    }

    let cndn = true;
    println!("{}", display_message(cndn));

    println!("{}", compare_with_five(3));
    println!("{}", compare_with_five(5));
    println!("{}", compare_with_five(7));

    let some_bool = true;

    match some_bool {
        true => println!("It's true"),
        false => println!("It's false"),
    }

    let some_int = 3;

    match some_int {
        1 => println!("It's one"),
        2 => println!("It's two"),
        3 => println!("It's three"),
        _ => println!("It's something else"),
    }

    display_num(1);
    display_num(2);
}

fn display_message(cndn: bool) -> String {
    if cndn {
        return "hello".to_string();
    }

    "goodbye".to_string()
}

fn compare_with_five(num: i32) -> String {
    if num < 5 {
        return "<5".to_string();
    } else if num > 5 {
        return ">5".to_string();
    } else {
        return "=5".to_string();
    }
}

fn display_num(num: i32) {
    match num {
        1 => println!("It's one"),
        2 => println!("It's two"),
        3 => println!("It's three"),
        _ => println!("It's something else"),
    }
}
