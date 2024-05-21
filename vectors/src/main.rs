/*
* Vectors
* - A vector contains a sequence of values of the same type.
* - Vectors are useful when you have a list of items that you want to process together.
* - Can add, remove and traverse elements.
*/
struct Test {
    score: i32,
}

fn main() {
    let my_scores = vec![
        Test { score: 90 },
        Test { score: 80 },
        Test { score: 70 },
        Test { score: 60 },
    ];

    for test in &my_scores {
        println!("Test score: {}", test.score);
    }

    let nums = vec![10, 20, 30, 40, 50];
    // Pointer is needed else it will take ownership of the vector
    for num in &nums {
        if num == &30 {
            println!("thirty");
        }
        println!("{}", num);
    }
    println!("{:?}", nums.len());

    for num in nums.iter() {
        match num {
            30 => println!("thirty"),
            _ => println!("{}", num),
        }
    }

    let mut my_numbers = Vec::new();
    my_numbers.push(10);
    my_numbers.push(20);

    println!("{:?}", my_numbers);
    my_numbers.push(12);
    my_numbers.push(15);
    println!("{:?}", my_numbers);

    let first = my_numbers[0];
    println!("First element: {}", first);
    my_numbers.pop();
    println!("{:?}", my_numbers);

    for number in my_numbers.iter() {
        println!("{}", number);
    }

    for number in &my_numbers {
        println!("{}", number);
    }

    for number in my_numbers.iter_mut() {
        *number *= 2;
    }

    println!("{:?}", my_numbers);

    let my_numbers = vec![1, 2, 3, 4, 5];

    let third: &i32 = &my_numbers[2];

    println!("The third element is {}", third);

    println!("The lenght of the vector {}", my_numbers.len());
}
