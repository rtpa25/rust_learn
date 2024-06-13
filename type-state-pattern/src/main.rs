/**
 * Leverage type system to encode state changes
 * Implemented by creating a type for each state
 * Use move semantics to invalidate a state
 * Return next state from previous state
 * Optionally drop the state
 * ex: Close a file, connection dropped, etc.
 * Compile time enforcement of logic
 */

struct BusTicket;
struct BoardBusTicket;

impl BusTicket {
    fn board(self) -> BoardBusTicket {
        println!("Boarding the bus");
        BoardBusTicket
    }
}

pub mod employee;
pub mod luggage;

fn main() {
    let ticket = BusTicket;
    let ticket = ticket.board();
    // let ticket = ticket.board(); // Error: value used here after move

    let employee_john = employee::Employee::new("John".to_string());
    let employee_marry = employee::Employee::new("Marry".to_string());

    let employee_john = employee_john.read_aggrement().sign();
    let employee_marry = employee_marry.read_aggrement().sign();

    match employee_john.train(70) {
        Ok(employee) => {
            employee.welcome();
        }
        Err(employee) => {
            let employee = employee.retry_training();
            match employee.train(80) {
                Ok(employee) => {
                    employee.welcome();
                }
                Err(employee) => {
                    employee.fire();
                }
            }
        }
    };

    match employee_marry.train(50) {
        Ok(employee) => {
            employee.welcome();
        }
        Err(employee) => {
            let employee = employee.retry_training();
            match employee.train(40) {
                Ok(employee) => {
                    employee.welcome();
                }
                Err(employee) => {
                    employee.fire();
                }
            }
        }
    };

    let luggage = luggage::Luggage::check_in(1, 10);
    let luggage = luggage.load();
    let luggage = luggage.offload();
    let luggage = luggage.await_pickup();
    luggage.end_custody();

    // let luggage = luggage.load(); // Error: value used here after move
}
