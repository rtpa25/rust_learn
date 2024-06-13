// Topic: Arrays & Slices
//
// Requirements:
// * Print pairs of numbers and their sums as they are streamed from a data source
// * If only one number is received, then print "Unpaired value: V",
//   where V is the value
//
// Notes:
// * A simulated data stream is already configured in the code
// * See the stdlib docs for the "chunks" method on "slice" for more info

use std::collections::HashMap;

fn data() -> &'static [u64] {
    &[5, 5, 4, 4, 3, 3, 1]
}

struct Contact {
    name: String,
    phone: String,
}

type ContactName = String;
type ContactKV = HashMap<ContactName, Contact>;

fn add_contact(index: &mut ContactKV, contact: Contact) {
    index.insert(contact.name.to_owned(), contact);
}

fn main() {
    let mut stream = data().chunks(2);

    let contactKV = &mut ContactKV::new();
    add_contact(
        contactKV,
        Contact {
            name: "John".to_string(),
            phone: "1234567890".to_string(),
        },
    );

    for chunk in stream {
        match chunk {
            [a, b] => {
                println!("{} + {} = {}", a, b, a + b);
            }
            [single] => {
                println!("Unpaired value: {}", single);
            }
            [] => {
                println!("No data");
            }
            _ => unreachable!(),
        }
    }

    let chars = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j'];

    match chars.as_slice() {
        [first, .., last] => {
            println!("First: {}, Last: {}", first, last);
        }
        [single] => {
            println!("this means slice to have only one element: {}", single);
        }
        [] => {
            println!("this means slice to be empty")
        }
    }

    match chars.as_slice() {
        [first, .., last, extra] => {
            println!("First: {}, Last: {}, Extra: {}", first, last, extra);
        }
        [one, two, ..] => {
            println!("First: {}, Second: {}", one, two);
        }
        [.., last] => {
            println!("Last: {}", last);
        }
        [] => {
            println!("this means slice to be empty")
        }
    }

    match chars.as_slice() {
        [first, ..] => {
            println!("First: {}", first);
        }
        // will never come here
        [.., last] => {
            println!("Last: {}", last);
        }
        [] => {
            println!("this means slice to be empty")
        }
    }

    // to ensure all arms are matched then put the largest one first
    match chars.as_slice() {
        [a, b, c, d, ..] => {
            println!("a: {}, b: {}, c: {}, d: {}", a, b, c, d);
        }
        [a, b, c, ..] => {
            println!("a: {}, b: {}, c: {}", a, b, c);
        }
        [a, b, ..] => {
            println!("a: {}, b: {}", a, b);
        }
        [a, ..] => {
            println!("a: {}", a);
        }
        [] => {
            println!("this means slice to be empty")
        }
    }

    let nums = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    match nums.as_slice() {
        [first @ 1, .., last @ 10] => {
            println!(
                "First: {} this should always be 1, Last: {} this should be always 10",
                first, last
            );
        }
        [first @ 1..=3, rest @ ..] => {
            println!(
                "First: {} this should be 1, 2 or 3, Rest: {:?}",
                first, rest
            );
        }
        [single] if single == &5 || single == &6 => {
            println!("this means slice to have only one element: {}", single);
        }
        [single] => {
            println!("this means slice to have only one element: {}", single);
        }
        [..] => {
            println!("this means slice to have more than one element");
        }
    }
}
