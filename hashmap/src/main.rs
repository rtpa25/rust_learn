use std::collections::HashMap;

struct Contents {
    content: String,
}

fn main() {
    let mut people = HashMap::new();
    people.insert("John", 32);
    people.insert("Jane", 34);
    people.insert("Freddy", 29);

    for (name, age) in &people {
        println!("{} is {} years old.", name, age);
    }

    match people.get("John") {
        Some(age) => println!("John is {} years old.", age),
        None => println!("No one found with that name."),
    }

    people.remove("Jane");
    people.insert("John", 33);

    for (name, age) in &people {
        println!("{} is {} years old.", name, age);
    }

    match people.get("John") {
        Some(age) => println!("John is {} years old.", age),
        None => println!("No one found with that name."),
    }

    let mut lockers = HashMap::new();
    lockers.insert(
        1,
        Contents {
            content: "Books".to_string(),
        },
    );
    lockers.insert(
        2,
        Contents {
            content: "Clothes".to_string(),
        },
    );
    lockers.insert(
        3,
        Contents {
            content: "Electronics".to_string(),
        },
    );
    lockers.insert(
        4,
        Contents {
            content: "Food".to_string(),
        },
    );

    for (number, contents) in &lockers {
        println!("Locker {} contains {}.", number, contents.content);
    }

    match lockers.get(&1) {
        Some(contents) => println!("Locker 1 contains {}.", contents.content),
        None => println!("No locker found with that number."),
    }
}
