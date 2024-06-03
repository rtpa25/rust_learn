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
pub mod db;
pub mod manager;
pub mod util;

fn main() {
    let db = db::Database::new();
    let mut manager = manager::Manager::new(db);
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
                    manager.add_bill();
                }
                "2" => {
                    println!("You choose to view bills");
                    manager.view_bills();
                }
                "3" => {
                    println!("You choose to remove bill");
                    manager.remove_bill();
                }
                "4" => {
                    println!("You choose to edit bill");
                    match manager.edit_bill() {
                        Ok(_) => println!("Bill edited successfully"),
                        Err(e) => {
                            println!("{}", e);
                            continue;
                        }
                    }
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
