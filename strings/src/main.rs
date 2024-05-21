/**
 * Two commonly used types of strings
 * String --> owned
 * &str --> borrowed string slice
 * Must use an owned String to store in a struct
 * Use &str when passing to a function
 */

struct Employee {
    name: String, // we can't take a reference here because the struct needs to own the data
                  // name: &str, // this will not work because when the struct goes out of runtime, it will try to clean up will not be able to because it doesn't own the data
}

fn print_it(data: &str) {
    println!("{:?}", data);
}

struct Person {
    name: String,
    age: u8,
    favourite_color: String,
}

impl Person {
    fn print_name_and_color(&self) {
        println!(
            "name: {}, favourite_color: {}",
            self.name, self.favourite_color
        );
    }
}

fn main() {
    print_it("Hello, World!");

    let owned_string = String::from("Hello, World!");
    print_it(&owned_string);

    let owned_string = "Hello, World!".to_string();
    print_it(&owned_string);

    let owned_string = "Hello, World!".to_owned();
    print_it(&owned_string);

    let employee = &mut Employee {
        name: "John Doe".to_string(),
    };
    println!("{:?}", employee.name);
    employee.name.push_str(" Jr.");

    print_it(&employee.name);

    let people = vec![
        Person {
            name: "John Doe".to_string(),
            age: 10,
            favourite_color: "blue".to_string(),
        },
        Person {
            name: "Jane Doe".to_string(),
            age: 25,
            favourite_color: "green".to_string(),
        },
        Person {
            name: "Joe Doe".to_string(),
            age: 6,
            favourite_color: "red".to_string(),
        },
    ];

    for person in &people {
        if person.age <= 10 {
            person.print_name_and_color();
        }
    }
}
