use std::{
    cell::{Cell, RefCell},
    rc::Rc,
    sync::Arc,
    thread,
};

use parking_lot::Mutex;
use rental::{create_rentals, Corporate, StoreFront, VeichleType};
use resturant::{new_table_order, Accounting, Chef, TableOrder, WaitStaff};

pub mod deadlock;
pub mod mutex_demo;
pub mod rental;
/**
 * Summary of borrow checker rules in Rust
 * Mutable Reference: You can have only one mutable reference to a variable at a time. This ensures that while a variable is being
 * modified, no other references can access it, preventing data races.
 *
 * Immutable References: You can have multiple immutable references to a variable at the same time. Immutable references allow
 * read-only access, so having multiple references is safe as none of them can modify the variable.
 *
 * Mixing Mutable and Immutable References: You cannot have a mutable reference if any immutable references exist. Similarly, you
 * cannot create immutable references if a mutable reference exists. This rule prevents simultaneous read and write access to a
 * variable, ensuring data consistency and preventing race conditions.
 */

/**
 * Smart pointers allow mutliple access to multiple owners
 * on same data
 * Reference counted - "Rc"
 *  Data deleted only when last owner is droped
 * Atomic reference counted - "Arc"
 *  Safe to use with mutlithreaded programs
 *
 *
 * Summary
 * Rc & Arc are used to share ownership
 * Data is dropped once all owners are dropped
 * Rc for single threading
 *  Rc::clone() to make a new reference
 * Arc for multi threading
 *  Arc::clone() to make a new reference
 */

/**
 * Mutable data is sometimes problematic
 *   Compiler errors, ownership issues etc.
 * Possible to create permanently mutable chunk of memory
 *    Less restrictive than compiler
 *    Trade off in implementation & performace
 *
 *
 * Cell & RefCell
 *
 * Cell
 * Permanently mutable memory location
 *  Can always be mutated even if the containing structure is immutable
 * Accessing cell data always results in a move or a copy of the data
 * Data should be copyable
 * #[derive(Copy, Clone)]
 * Inefficient for large data types
 *  Limit to numbers, bools, chars, etc.
 * Prefer mutable references for large data types
 *
 * RefCell
 * Permanently mutable memory location
 *  Can always be mutated even if the containing structure is immutable
 * Accessing RefCell data alkays results in a borrow
 *  Effecient data access compared to cell
 *  Borrow checked at runtime
 *    Will panic at runtime if rules are violated
 *    Only one mutable borrow at a time
 * Prefer &mut
 *
 * Both cell and refcell Not thread-safe,
 * and will give a compile time error  if done
 */

/**
 * Synchoronization
 * Data needs to be synchronized for safe access
 * Common sync primitive is a mutex
 *  mutually excliusive lock
 * Uses atomic operations to ensure that data is only accessed by one thread at a time
 *  Atomic operations are "all or nothing" operation, enforced by the CPU
 *  Data stays consistent
 *
 *
 * Mutex
 * Mutex wrap data, making data mutually exclusive
 *  Only one thread can access at a time
 *  All other threads will wait until finished
 * Mutexes cannot be shared among threads
 *  Wrap with a smart pointer (Arc
 *  Share the Arc among threads
 * Use parking_lot crate for more efficient mutexes
 *  Better API and per that std lib
 *
 * Suggested to do heavy computation before locking the mutex
 * During locking phase just copy/clone data then unlock
 * so locking time for otgher threads will be minimal
 */
// ┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓v
// ┃ How Mutex Works: Locks ┃
// ┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛

// Thread A     Thread B
//  ↓              ↓

// ┌ Lock ───────┐
// │             │
// │     ┌ Try Lock ┐
// └────▶│          │
//       │   Wait   │
//       └────────┘

// ┌ Unlock ─────┐
// │             │
// │     ┌ Lock ┐
// └────▶│      │
//       │ Unlock │
//       └──────┘

/**
 * Deadlocks
 * A deadlock is a situation where locks are waiting for one another
 * Threads become "stuck" and are unable to continue
 * Deadlocks can occur when:
 *  Using mutliple locks
 *  Recursing while taking a lock
 *  Locking the same lock twice
 */
pub mod resturant;

struct Veichle {
    id: i32,
}

struct Door {
    vecihle: Rc<Veichle>,
}

fn main() {
    let car = Rc::new(Veichle { id: 1 });

    let left_door = Door {
        vecihle: Rc::clone(&car),
    };
    let right_door = Door {
        vecihle: Rc::clone(&car),
    };

    println!("Car id: {}", car.id);

    // Droping the car
    drop(car);

    // This will not work as car is already droped
    // println!("Car id: {}", car.id);
    // but car can still be accessed through left_door and right_door
    println!("Left door car id: {}", left_door.vecihle.id);
    println!("Right door car id: {}", right_door.vecihle.id);

    // Droping the left door
    drop(left_door);

    // This will not work as left_door is already droped
    // println!("Left door car id: {}", left_door.vecihle.id);

    // Droping the right door
    drop(right_door);

    // This will not work as right_door is already droped
    // println!("Right door car id: {}", right_door.vecihle.id);

    // At this point car is droped and no longer accessible

    let my_book = Book {
        signed: Cell::new(false),
    };

    println!("Book is signed: {}", my_book.is_signed());
    my_book.sign();
    println!("Book is signed: {}", my_book.is_signed());

    let name = "amie".to_owned();
    let person = Person {
        name: RefCell::new(name),
    };

    {
        let mut name = person.name.borrow_mut();
        println!("Name: {}", name);

        *name = "Time".to_owned();
        println!("Name: {}", name);
    }

    // this will not work as name is already borrowed mutably but becasue it's inside a refcell it's catched at compile time
    // and panics at runtime. So follow borrow checker's rules all the time to avoid runtime panics. To fix this the scoping we have done right now is done to help. So we can't borrow mutably again, because when we reach at this place the mutable borrow is already droped.
    {
        person.name.replace("Time".to_owned());
    }

    let name = person.name.try_borrow();
    match name {
        Ok(name) => println!("Name: {}", name),
        Err(_) => println!("Name is already borrowed mutably"),
    }

    let orders = Rc::new(RefCell::new(Vec::new() as Vec<TableOrder>));
    let chef = Chef(Rc::clone(&orders));
    let wait_staff = WaitStaff(Rc::clone(&orders));
    let accounting = Accounting(Rc::clone(&orders));

    let order = new_table_order();

    {
        orders.borrow_mut().push(order);
    }

    dbg!(chef.0.borrow());
    drop(chef);
    dbg!(wait_staff.0.borrow());
    drop(wait_staff);
    dbg!(accounting.0.borrow());

    let rentals = create_rentals();
    let corporate = Corporate(Rc::clone(&rentals));
    let store_front = StoreFront(Rc::clone(&rentals));

    store_front.rent_vehicle("1");
    corporate.take_for_maintenance("1");

    corporate.add_rental(VeichleType::Car, "10".to_owned());
    store_front.rent_vehicle("10");

    corporate.take_for_maintenance("3");

    let rentals = corporate.get_rentals();

    for rental in rentals {
        dbg!(rental);
    }

    let counter = Counter(0);
    let shared_counter = Arc::new(Mutex::new(counter));

    let thread_1_counter = Arc::clone(&shared_counter);
    let thread_2_counter = shared_counter.clone();

    let th1 = thread::spawn(move || {
        let mut counter = thread_1_counter.lock();
        counter.0 += 100;
        println!("Counter from thread1: {}", counter.0)
    });

    let th2 = thread::spawn(move || {
        let mut counter = thread_2_counter.lock();
        counter.0 -= 20;
        println!("Counter from thread2: {}", counter.0)
        // Once this scope ends the lock is released
    });

    th1.join().and_then(|_| th2.join()).unwrap();

    let counter = shared_counter.lock();
    println!("Counter from main thread: {}", counter.0);
}

struct Counter(usize);

#[derive(Debug)]
struct Book {
    signed: Cell<bool>,
}

impl Book {
    fn sign(&self) {
        self.signed.set(true);
    }

    fn is_signed(&self) -> bool {
        self.signed.get()
    }
}

struct Person {
    name: RefCell<String>,
}
