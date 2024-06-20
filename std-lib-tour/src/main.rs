use std::{cmp::Ordering, collections::HashMap, ops::Add};

use fruit_stand::Fruit;

pub mod fruit_stand;
pub mod iter_activity;

// ordered by the variants instead which means client services is less than marketing and marketing is less than ops.
#[derive(PartialEq, PartialOrd)]
enum Floor {
    ClientServices,
    Marketing,
}

#[derive(PartialEq)]
struct User {
    id: u64,
    name: String,
}

impl PartialOrd for User {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.name < other.name {
            Some(Ordering::Less)
        } else if self.name > other.name {
            Some(Ordering::Greater)
        } else {
            Some(Ordering::Equal)
        }
    }
}

/**
 * Operator Overloading
 * Operators can be overloaded for structs and enums
 * Trait implementation required
 * All overloadable operators are available in std::ops module
 * Behvaior should be consistent with the operator's original meaning
 *  Adding should make something larger
 *  Subtracting should make something smaller
 */

struct Speed(u32);

impl Add<Self> for Speed {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Speed(self.0 + rhs.0)
    }
}

impl Add<u32> for Speed {
    type Output = Self;

    fn add(self, rhs: u32) -> Self::Output {
        Speed(self.0 + rhs)
    }
}

fn main() {
    let first = Floor::ClientServices;
    let second = Floor::Marketing;

    if first == second {
        println!("The first and second floors are the same.");
    } else {
        println!("The first and second floors are different.");
    }

    if first < second {
        println!("The first floor is below the second floor.");
    } else {
        println!("The first floor is above the second floor.");
    }

    let user1 = User {
        id: 1,
        name: "Alice".to_string(),
    };
    let user2 = User {
        id: 2,
        name: "Bob".to_string(),
    };

    // for equal to be true both fields must be equal
    if user1 == user2 {
        println!("The users are the same.");
    } else {
        println!("The users are different.");
    }

    // if we use the default paritalOrd implementation we can compare the users only first property is used so id
    // but in thise case we have manually implemented partialOrd so we can compare the users by name
    if user1 < user2 {
        println!("User 1 is less than user 2.");
    } else {
        println!("User 1 is greater than user 2.");
    }

    let speed1 = Speed(10);
    let speed2 = Speed(20);

    let total_speed = speed1 + speed2;
    println!("Total speed: {}", total_speed.0);

    let speed3 = Speed(30);
    let total_speed = 10 + speed3.0;
    println!("Total speed: {}", total_speed);

    let mut odd = Odd::new(10);

    while let Some(n) = odd.next() {
        println!("{}", n);
    }

    let odds = Odd::new(10);

    for n in odds {
        println!("{}", n);
    }

    let evens = Odd::new(10);

    for e in evens.map(|n| n + 1) {
        println!("{}", e);
    }

    let mut friends = Friends {
        names: vec!["Alice".to_string(), "Bob".to_string()],
    };

    for friend in &friends {
        println!("{}", friend);
    }
    for friend in &friends {
        println!("{}", friend);
    }
    for friend in &mut friends {
        if friend == "Alice" {
            *friend = "Alice 2".to_string();
        }
    }

    let mut map = HashMap::new();

    // set the value of the key "Alice" to 1
    map.insert("Alice", 1);
    map.insert("Brian", 2);

    for (key, value) in map {
        println!("{}: {}", key, value);
    }

    let mut fruits = HashMap::new();
    fruits.insert(Fruit::Apple, 10);
    fruits.insert(Fruit::Banana, 20);
    fruits.insert(Fruit::Orange, 30);

    let mut fruit_stand = fruit_stand::FruitStand { fruits };

    for (fruit, count) in &fruit_stand {
        println!("{:?}: {}", fruit, count);
    }

    for (fruit, count) in &fruit_stand {
        println!("{:?}: {}", fruit, count);
    }

    for (fruit, count) in &mut fruit_stand {
        if fruit == &Fruit::Banana {
            *count -= 10;
        }
        println!("{:?}: {}", fruit, count);
    }

    let mut multiplier = iter_activity::Multiplier::new();

    for _ in 0..5 {
        println!("Multiplier: {}", multiplier.next().unwrap());
    }

    multiplier.powerup(2);

    for _ in 0..5 {
        println!("Multiplier: {}", multiplier.next().unwrap());
    }
}

struct Odd {
    number: isize,
    max: isize,
}

impl Iterator for Odd {
    type Item = isize;

    fn next(&mut self) -> Option<Self::Item> {
        self.number += 2;
        if self.number <= self.max {
            Some(self.number)
        } else {
            None
        }
    }
}

impl Odd {
    fn new(max: isize) -> Self {
        Self { number: -1, max }
    }
}

struct Friends {
    names: Vec<String>,
}

impl IntoIterator for Friends {
    type IntoIter = std::vec::IntoIter<Self::Item>;
    type Item = String;

    fn into_iter(self) -> Self::IntoIter {
        self.names.into_iter()
    }
}

impl<'a> IntoIterator for &'a Friends {
    type IntoIter = std::slice::Iter<'a, String>;
    type Item = &'a String;

    fn into_iter(self) -> Self::IntoIter {
        self.names.iter()
    }
}

impl<'a> IntoIterator for &'a mut Friends {
    type IntoIter = std::slice::IterMut<'a, String>;
    type Item = &'a mut String;

    fn into_iter(self) -> Self::IntoIter {
        self.names.iter_mut()
    }
}
