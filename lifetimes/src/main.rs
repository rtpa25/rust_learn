/**
 * Lifetimes
 * - Lifetimes annotations indicate that there exists some owned data
 * that:
 * - Lives at least as long as the borrowed data
 * - Outlives outlasts the scope of a borrow
 * - Exists longer than the scope of a borrow
 *
 * Strucutures utilizing borrowed data must:
 * - Always be created after the owner was created
 * - Always be destroyed before the owner is destroyed
 *
 * Lifetimes allow:
 * - Borrowed data to be used in a struct
 * - Returning references from functions
 *
 * Lifetimes are the mechanism that tracks how long
 * a piece of data resides in memory
 * Lifetimes are usually elided, but can be specified manually
 * Lifetimes will be checked by the compiler
 */
#[derive(Debug)]
struct Cards {
    innner: Vec<IdCard>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum City {
    Barlang,
    Kigali,
    NewYork,
}

#[derive(Debug)]
struct IdCard {
    name: String,
    city: City,
    age: u8,
}
impl IdCard {
    fn new(name: String, city: City, age: u8) -> IdCard {
        IdCard { name, city, age }
    }
}

fn new_ids() -> Vec<IdCard> {
    vec![
        IdCard::new("John".to_string(), City::NewYork, 41),
        IdCard::new("Jane".to_string(), City::Kigali, 21),
        IdCard::new("Doe".to_string(), City::Barlang, 80),
        IdCard::new("asd".to_string(), City::Barlang, 23),
        IdCard::new("fd".to_string(), City::NewYork, 19),
        IdCard::new("gfdh".to_string(), City::Kigali, 10),
        IdCard::new("wrt".to_string(), City::Barlang, 40),
        IdCard::new("Doe".to_string(), City::Barlang, 45),
    ]
}

#[derive(Debug)]
struct YoungPeople<'a> {
    inner: Vec<&'a IdCard>,
}

impl<'a> YoungPeople<'a> {
    fn living_in_kigali(&self) -> Self {
        Self {
            inner: self
                .inner
                .iter()
                .filter(|id_cards| id_cards.city == City::Kigali)
                .map(|id_cards| *id_cards)
                .collect(),
        }
    }
}

// 'static indicates that this constant will live for the entire duration of the program
const MOCK_DATA: &'static str = include_str!("../mock-data.csv");

struct Employee<'a> {
    name: &'a str,
    title: &'a str,
}

struct EmployeeDirectory<'a> {
    employees: Vec<Employee<'a>>,
}

impl EmployeeDirectory<'_> {
    fn print_employees(&self) {
        for employee in self.employees.iter() {
            let data = (employee.name, employee.title);
            println!("{:?}", data);
        }
    }
}

fn main() {
    let cards = Cards { innner: new_ids() };

    let young = YoungPeople {
        inner: cards
            .innner
            .iter()
            .filter(|id_cards| id_cards.age < 25)
            .collect(),
    };

    for id_cards in cards.innner.iter() {
        println!("{:?}", id_cards);
    }

    for id_cards in young.inner.iter() {
        println!("{:?}", id_cards);
    }

    for id_cards in young.living_in_kigali().inner.iter() {
        println!("{:?}", id_cards);
    }

    let employee_directory = EmployeeDirectory {
        employees: MOCK_DATA
            .lines()
            .skip(1)
            .map(|line| {
                let data: Vec<&str> = line.split(',').collect();
                Employee {
                    name: data[1],
                    title: data[4],
                }
            })
            .collect(),
    };

    employee_directory.print_employees();

    let short = "short";
    let long = "longer";

    println!("{}", longest(short, long));
}

// here tick a is a lifetime annotation, and annotates that we are returning a reference to a string that has the same lifetime as the input strings
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
