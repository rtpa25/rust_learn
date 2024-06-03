// Project 1: Interactive bill manager
//
// Summary:
//   Create a command line bills/expenses manager that runs
//   interactively. This mini project brings together many of
//   the concepts learn thus far into a single application.
//
//   The user stories/requirements are split into stages.
//   Fully implement each stage as a complete working program
//   before making changes for the next stage. Leverage the
//   compiler by using `cargo check --bin p1` when changing
//   between stages to help identify adjustments that need
//   to be made.
//
// User stories:
// * Stage 1:
//   - I want to add bills, including the name and amount owed.
//   - I want to view existing bills.
// * Stage 2:
//   - I want to remove bills.
// * Stage 3:
//   - I want to edit existing bills.
//   - I want to go back if I change my mind.
//
// Tips:
// * Use the loop keyword to create an interactive menu.
// * Each menu choice should be it's own function, so you can work on the
//   the functionality for that menu in isolation.
// * A vector is the easiest way to store the bills at stage 1, but a
//   hashmap will be easier to work with at stages 2 and 3.

pub mod bill;
pub mod util;

fn main() {
    let mut db = bill::Database::new();
    println!("==============Welcome to Bill Manager==============");
    loop {
        println!("Hit 1. Add bill");
        println!("Hit 2. View bills");
        println!("Hit 3. Remove bill");
        println!("Hit 4. Edit bill");
        println!("Hit 5. Exit");

        let choice = util::collect_user_input();
        match choice {
            Ok(val) => match val.as_str() {
                "1" => {
                    println!("You choose to add a bill");
                    println!("Enter bill name: ");
                    let name = match util::collect_user_input() {
                        Ok(val) => val,
                        Err(e) => panic!("Error occured while reading user data: {:?}", e),
                    };
                    println!("Enter bill amount: ");
                    let amount = match util::collect_user_input() {
                        Ok(val) => match val.parse::<f64>() {
                            Ok(val) => val,
                            Err(e) => panic!("Error occured while parsing amount: {:?}", e),
                        },
                        Err(e) => panic!("Error occured while reading user data: {:?}", e),
                    };

                    db.add_bill(name, amount);
                    println!("Bill added successfully")
                }
                "2" => {
                    println!("You choose to view bills");
                    db.view_bills();
                }
                "3" => {
                    println!("You choose to remove bill");
                    db.view_bills();
                    println!("Enter bill id you wanna remove: ");
                    let id = match util::collect_user_input() {
                        Ok(val) => val,
                        Err(e) => panic!("Error occured while reading user data: {:?}", e),
                    };
                    db.remove_bill(&id);
                    println!("Bill removed successfully");
                }
                "4" => {
                    println!("You choose to edit bill");
                    db.view_bills();
                    println!("Enter bill id you wanna edit: ");
                    let id = match util::collect_user_input() {
                        Ok(val) => val,
                        Err(e) => panic!("Error occured while reading user data: {:?}", e),
                    };
                    let bill = db.get_bill(&id);
                    if bill.is_none() {
                        println!("Bill not found");
                        continue;
                    }

                    println!("Enter new amount, for bill id: {}", id);
                    println!("hit enter to keep the same amount");
                    let mut amount = match util::collect_user_input() {
                        Ok(val) => match val.parse::<f64>() {
                            Ok(val) => Some(val),
                            Err(_) => None,
                        },
                        Err(_) => None,
                    };
                    if amount.is_none() {
                        amount = Some(bill.unwrap().amount);
                    }

                    println!("Enter new name, for bill id: {}", id);
                    println!("hit enter without anything to keep the same name");
                    let mut name = match util::collect_user_input() {
                        Ok(val) => {
                            if val.is_empty() {
                                None
                            } else {
                                Some(val)
                            }
                        }
                        Err(_) => None,
                    };
                    if name.is_none() {
                        name = Some(bill.unwrap().name.clone());
                    }

                    if name == Some(bill.unwrap().name.clone())
                        && amount == Some(bill.unwrap().amount)
                    {
                        println!("No changes made");
                        continue;
                    }

                    println!(
                        "Bill with id: {} will be updated to amount: {:?} and name: {:?}",
                        id, amount, name
                    );
                    println!("Are you sure you want to update? (y/n)");
                    let confirm = match util::collect_user_input() {
                        Ok(val) => val,
                        Err(e) => panic!("Error occured while reading user data: {:?}", e),
                    };
                    if confirm != "y" {
                        println!("Exiting...");
                        continue;
                    }
                    db.edit_bill(&id, amount, name);
                    println!("Bill updated successfully");
                }
                "5" => {
                    println!("Exiting...");
                    break;
                }
                _ => {
                    println!("Invalid choice");
                }
            },
            Err(e) => {
                panic!("Error: {:?}", e)
            }
        }
    }
    println!("==============Goodbye==============")
}
