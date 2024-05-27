/*
* Result is a data type that contains one of two possible outcomes:
* Ok: Represents success and contains a value
* Err: Represents an error and contains a value
* Used in functions that may fail
* ex: File I/O, network requests, etc.
*/

#[derive(Debug)]
enum MenuChoice {
    Soup,
    Salad,
    Pasta,
    Beef,
    Fish,
    Quit,
}

fn get_choice(input: &str) -> Result<MenuChoice, String> {
    match input {
        "s" => Ok(MenuChoice::Soup),
        "sa" => Ok(MenuChoice::Salad),
        "p" => Ok(MenuChoice::Pasta),
        "b" => Ok(MenuChoice::Beef),
        "f" => Ok(MenuChoice::Fish),
        "q" => Ok(MenuChoice::Quit),
        _ => Err("Invalid choice".to_string()),
    }
}

fn main() {
    run_resturant();

    let adult1 = Adult::new("John", 24);

    match adult1 {
        Ok(adult) => {
            println!("Welcome to the club: {:?}", adult);
        }
        Err(e) => {
            println!("You are not an adult: {}", e);
        }
    }

    let adult2 = Adult::new("Jane", 20);

    match adult2 {
        Ok(adult) => {
            println!("Welcome to the club: {:?}", adult);
        }
        Err(e) => {
            println!("You are not an adult: {}", e);
        }
    }
}

fn run_resturant() {
    println!("What would you like to eat?");
    println!("s) Soup");
    println!("sa) Salad");
    println!("p) Pasta");
    println!("b) Beef");
    println!("f) Fish");
    println!("q) Quit");

    let mut vec: Vec<MenuChoice> = Vec::new();

    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        let choice = pick_choice(input);
        match choice {
            Ok(MenuChoice::Quit) => {
                println!("coming right up: {:?}", vec);
                break;
            }
            Ok(val) => {
                println!("Great choice!: {:?} ", val);
                vec.push(val);
            }
            Err(e) => {
                println!("this menu item does not exist{}", e);
            }
        }
    }
}

fn pick_choice(input: &str) -> Result<MenuChoice, String> {
    // here we can use the question mark operator and not inside the main function because on error it will return the error to the main function which is not what we want, we want to return the error to the caller of this function
    // so ? operator can only be used when the return type of the function is Result
    let choice = get_choice(input)?;
    return Ok(choice);
}

#[derive(Debug)]
struct Adult {
    name: String,
    age: u8,
}

impl Adult {
    fn new(name: &str, age: u8) -> Result<Self, String> {
        match age {
            age if age >= 21 => Ok(Adult {
                name: name.to_string(),
                age,
            }),
            _ => Err("You are not an adult".to_string()),
        }
    }
}
