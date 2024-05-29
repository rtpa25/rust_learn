// use std::array;

// use rand::{distributions::Slice, Rng};
/**
 * Basic Closures
 * closures are function inside a function
 */
// fn main() {
//     let sum = add_fn(1, 2);
//     println!("sum: {}", sum);

//     let add = |a: i32, b: i32| -> i32 { a + b };
//     let sum2 = add(1, 2);

//     println!("sum2: {}", sum2);

//     // inferring the type by usage
//     let add = |a, b| a + b; // better to explicitly define the type
//     let sum3 = add(1, 2);
//     println!("sum3: {}", sum3);

//     let add_one = |x| x + 1;
//     let sum4 = add_one(1);
//     println!("sum4: {}", sum4);
// }

// fn add_fn(a: i32, b: i32) -> i32 {
//     a + b
// }

/**
 * Map Combinator
 */
// fn maybe_num() -> Option<i32> {
//     let random_number = generate_random_number(0, 10);
//     if random_number > 5 {
//         return None;
//     }
//     Some(9)
// }

// fn maybe_word(s: &str) -> Option<String> {
//     let random_number = generate_random_number(0, 10);
//     if random_number > 5 {
//         return None;
//     }
//     Some(s.to_string())
// }

// // Function to generate a random number within a given range
// fn generate_random_number(min: i32, max: i32) -> i32 {
//     let mut rng = rand::thread_rng(); // Create a random number generator
//     rng.gen_range(min..=max) // Generate a random number in the range [min, max]
// }

// fn main() {
//     let plus_one = match maybe_num() {
//         Some(num) => Some(num + 1),
//         None => None,
//     };

//     let plus_one = maybe_num().map(|n| n + 1);

//     let word_length_times_2 = maybe_word("hello world").map(|s| s.len()).map(|l| l * 2);

//     let username = generate_random_username();
//     let user = find_user(&username).map(|id| User {
//         user_id: id,
//         name: username,
//     });

//     match user {
//         Some(u) => println!("Found user with id: {} & name: {}", u.user_id, u.name),
//         None => println!("not found"),
//     };
// }

// struct User {
//     user_id: i32,
//     name: String,
// }

// fn find_user(name: &str) -> Option<i32> {
//     let name = name.to_lowercase();

//     match name.as_str() {
//         "sam" => Some(1),
//         "matt" => Some(5),
//         "katie" => Some(9),
//         _ => None,
//     }
// }

// fn generate_random_username() -> String {
//     let possible_usernames = ["sam", "matt", "katie"];

//     let name_idx = generate_random_number(0, 5) as usize;

//     let n = match possible_usernames.get(name_idx) {
//         Some(val) => val,
//         None => "random_name",
//     };

//     return n.to_string();
// }

/**
 * Option combinator pattern
 */
// fn main() {
//     let a: Option<i32> = Some(1);

//     let a_is_some = a.is_some();
//     dbg!(a_is_some);

//     let a_in_none = a.is_none();
//     dbg!(a_in_none);

//     let a_mapped = a.map(|n| n + 1); // if no data nothing happens
//     dbg!(a_mapped);

//     let a_filtered = a.filter(|n| n == &1); // get rid of some data if internal closure returns false
//     dbg!(a_filtered);

//     let a_or_else = a.or_else(|| Some(5)); // if a is Some() then nothing happens if none then return value returned from closure passed into or_else -> the closure has to return an optional value;
//     dbg!(a_or_else);

//     let a_unwrap_or_else = a.unwrap_or_else(|| 0); // closure needs to return an unwrapped or a normal value, if value a is Some() then it is unwapped if None the default return of closure is passed on
//     dbg!(a_unwrap_or_else);
// }

/**
 * Iterators
 */
// fn main() {
//     let numbers = vec![1, 2, 3, 4, 5];

//     // iterate over each element of numbers then map and convert then collect inside the new variable
//     let plus_one_nums: Vec<i32> = numbers.iter().map(|n| n + 1).collect();

//     let new_nums: Vec<i32> = numbers.iter().filter(|&x| x <= &3).map(|y| *y).collect();

//     let find_me = numbers.iter().find(|&x| x == &3);

//     let count = numbers.iter().count();

//     let min = numbers.iter().min();
//     let max = numbers.iter().max();

//     let take: Vec<&i32> = numbers.iter().take(3).collect();

//     let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];

//     let trippled: Vec<i32> = data.iter().map(|n| n * 3).filter(|n| n > &10).collect();

//     for val in trippled {
//         println!("{}", val);
//     }
// }

/**
 * Ranges
*/
fn main() {
    let range1 = 1..=3;
    for num in range1 {
        println!("{}", num)
    }

    let range2 = 1..4;
    for num in range2 {
        println!("{}", num)
    }

    let ch_range = 'a'..'h';
    for ch in ch_range {
        println!("{}", ch)
    }

    let maybe_user = Some("Jerry");
    // this is same as match
    if let Some(u) = maybe_user {
        println!("{}", u)
    } else {
        println!("no user")
    }

    let red = Colors::Red;

    if let Colors::Red = red {
        println!("this is red{:?}", red)
    }

    let mut data = Some(3);

    while let Some(i) = data {
        println!("loop");
        data = None;
    }

    let nums = vec![1, 2, 3, 4];
    let mut nums_iter = nums.iter();

    while let Some(num) = nums_iter.next() {
        println!("num = {:?}", num);
    }
}

#[derive(Debug)]
enum Colors {
    Red,
    Blue,
    Green,
}
