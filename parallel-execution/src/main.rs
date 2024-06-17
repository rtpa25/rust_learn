use colored::*;
use crossbeam_channel::{unbounded, Receiver};
use std::{
    thread::{self, JoinHandle},
    time::Duration,
};

/**
 * Thread Basics
 * A thread uses serial execution
 *  each line of code is executed one at a time
 * Multicore cpus can have mutliple threads
 *  Threads still executes serially
 * Each thread can execute different tasks
 *  hence providing better cpu utilization
 * Threads are isolated from one another
 *  Require additional work to communicate
 *  Should communicate infrequently for performance reasons,
 *  over communication can lead to multi threaded programs running slower
 *  than single threaded programs
 */

/**
 * Threads are spawned (Created)
 *  Threads can spawn threads
 *  use the main thread for spawning in most cases
 *  fn main() is the main thread
 * Code is no longer executed line-by-line with threads
 *  Require careful planning
 * When a thread completes work, it should be "joined" back into
 * the main thread
 *  Ensures that the thread has completed
 */

/**
 * Threads have "thread-local" memory
 *  Owned by the thread
 *  Only accessible in the thread
 * Data can be copied or moved into threads
 *  Can be done when thread is created
 *  Becomes thread-local once moved into the thread
 */

/**
 * Threads are non-deterministic
 *  Execution order will vary each time the program runs
 * Ending the main thread will terminate all spawned threads
 *  Join on the main thread to wait for threads to complete
 * Each thread has it's local chunk of memory
 */

/**
 * Channels
 * One way communication between threads
 *  Message passing
 *  Sender and receiver
 * Can have limited or unlimited capacity
 * Message Passing
 * enum commonly used for messages
 *  match allows easy message handling
 * Guraanteed in-order delivery
 * Can be blocking or non-blocking
 *  Block on sender: channel full
 *  Block on receiver: No messages
 *  Behaviour determined by function not by channel
 */

fn math(a: i32, b: i32, op: Box<dyn Fn(i32, i32) -> i32>) -> i32 {
    op(a, b)
}

fn main() {
    let name = "Rust";
    let add = Box::new(move |a, b| {
        // moving the name variable from the main function to the closure
        // so that the closure can use the name variable
        println!("Hello, {}", name);
        a + b
    });
    let sub = |a, b| a - b;
    let mul = |a, b| a * b;
    let div = |a, b| a / b;

    let a = 10;
    let b = 5;

    println!("{} + {} = {}", a, b, math(a, b, add));
    println!("{} - {} = {}", a, b, math(a, b, Box::new(sub)));
    println!("{} * {} = {}", a, b, math(a, b, Box::new(mul)));
    println!("{} / {} = {}", a, b, math(a, b, Box::new(div)));

    let handle = thread::spawn(move || {
        println!("Hello from a thread!");
    });
    match handle.join() {
        Err(e) => println!("Error: {:?}", e),
        Ok(_) => println!("Thread finished successfully"),
    }

    let interation = 10;
    let a = thread::spawn(move || {
        for i in 1..interation {
            println!("Thread A: {}", i);
        }
    });

    let b = thread::spawn(move || {
        for i in 1..interation {
            println!("Thread B: {}", i);
        }
    });

    // unwrap is safe here cause we know the function will never fail
    // Order of join does not matter here, and the order is actually non deterministic
    b.join().unwrap();
    a.join().unwrap();

    let value = thread::spawn(move || {
        thread::sleep(Duration::from_secs(1));
        42
    });

    println!("Waiting on thread");

    match value.join() {
        Ok(v) => println!("Thread returned: {}", v),
        Err(e) => println!("Error: {:?}", e),
    }

    let data = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j'];

    let caps = thread::spawn(move || {
        let data: Vec<char> = data.iter().map(|c| c.to_ascii_uppercase()).collect();
        data
    });

    match caps.join() {
        Ok(v) => println!("Thread returned: {:?}", v),
        Err(e) => println!("Error: {:?}", e),
    }

    let t1 = thread::spawn(|| msg_hello());
    let mut s1 = "";
    let t2 = thread::spawn(|| msg_thread());
    let mut s2 = "";
    let t3 = thread::spawn(|| msg_excited());
    let mut s3 = "";

    match t1.join() {
        Ok(v) => s1 = v,
        Err(e) => println!("Error: {:?}", e),
    }

    match t2.join() {
        Ok(v) => s2 = v,
        Err(e) => println!("Error: {:?}", e),
    }

    match t3.join() {
        Ok(v) => s3 = v,
        Err(e) => println!("Error: {:?}", e),
    }

    println!("{}{}{}", s1, s2, s3);

    let (worker_tx, worker_rx) = unbounded();
    let (main_tx, main_rx) = unbounded();

    let handler1 = thread::spawn(move || loop {
        match worker_rx.recv() {
            Ok(msg) => match msg {
                WorkerMessage::PrintData(data) => println!("{}", data),
                WorkerMessage::Sum(a, b) => {
                    let result = a + b;
                    println!("Worker summing: {}", result);
                    // when we send data to a channel the data ownership is transfered to the channel
                    main_tx.send(MainMessage::SumResult(result)).unwrap();
                }
                WorkerMessage::Quit => {
                    println!("Worker terminated");
                    main_tx.send(MainMessage::WorkerQuit).unwrap();
                    break;
                }
            },
            Err(_) => {
                println!("Error receiving message");
                break;
            }
        }
    });

    worker_tx
        .send(WorkerMessage::PrintData("Hello, from main!".to_string()))
        .unwrap();
    worker_tx.send(WorkerMessage::Sum(1, 2)).unwrap();
    worker_tx.send(WorkerMessage::Quit).unwrap();

    while let Ok(msg) = main_rx.recv() {
        match msg {
            MainMessage::SumResult(result) => println!("Main Result: {}", result),
            MainMessage::WorkerQuit => {
                println!("Main worker teerminated");
                break;
            }
        }
    }

    handler1.join().unwrap();

    let (light_tx, light_rx) = unbounded();
    let light = spawn_light_thread(light_rx);

    light_tx.send(LightMsg::On).unwrap();
    light_tx.send(LightMsg::ChangeColor(255, 0, 0)).unwrap();
    light_tx.send(LightMsg::ChangeColor(0, 255, 0)).unwrap();
    light_tx.send(LightMsg::ChangeColor(0, 0, 255)).unwrap();
    light_tx.send(LightMsg::Off).unwrap();
    light_tx.send(LightMsg::Disconnect).unwrap();

    let light_status = light.join().unwrap();
    println!("Light status: {:?}", light_status);
}

// Topic: Multithreading
//
// Requirements:
// * Run the provided functions in threads
// * Retrieve the data from the threads to print the message
//   "Hello, threads!"
//
// Notes:
// * Use the join function to wait for threads to finish

fn msg_hello() -> &'static str {
    std::thread::sleep(Duration::from_millis(1000));
    "Hello, "
}

fn msg_thread() -> &'static str {
    std::thread::sleep(Duration::from_millis(1000));
    "threads"
}

fn msg_excited() -> &'static str {
    std::thread::sleep(Duration::from_millis(1000));
    "!"
}

enum WorkerMessage {
    PrintData(String),
    Sum(i32, i32),
    Quit,
}

enum MainMessage {
    SumResult(i32),
    WorkerQuit,
}

#[cfg(test)]
mod test {
    use super::*;
    use crossbeam_channel::unbounded;

    #[test]
    fn light_off_when_disconnect() {
        let (s, r) = unbounded();

        let light = spawn_light_thread(r);
        s.send(LightMsg::Disconnect).expect("channel disconnected");

        let light_status = light.join().expect("failed to join light thread");

        if let LightStatus::On = light_status {
            panic!("light should be off after disconnection");
        }
    }

    #[test]
    fn light_off_when_dropped() {
        let (s, r) = unbounded();

        let light = spawn_light_thread(r);
        drop(s);

        let light_status = light.join().expect("failed to join light thread");

        if let LightStatus::On = light_status {
            panic!("light should be off after dropping sender");
        }
    }
}

enum LightMsg {
    // Add additional variants needed to complete the exercise
    ChangeColor(u8, u8, u8),
    Disconnect,
    Off,
    On,
}

#[derive(Debug, PartialEq)]
enum LightStatus {
    Off,
    On,
}

fn spawn_light_thread(receiver: Receiver<LightMsg>) -> JoinHandle<LightStatus> {
    thread::spawn(move || {
        let mut status = LightStatus::Off;
        loop {
            match receiver.recv() {
                Ok(msg) => match msg {
                    LightMsg::ChangeColor(r, g, b) => {
                        println!(
                            "Light is now {}",
                            if status == LightStatus::On {
                                "on"
                            } else {
                                "off"
                            }
                        );
                        println!(
                            "Light color changed to: {}",
                            format!(
                                "{}{}{}",
                                "RGB(".green(),
                                format!("{}, {}, {}", r, g, b).yellow(),
                                ")".green()
                            )
                        );
                        status = LightStatus::On;
                    }
                    LightMsg::On => {
                        println!("Light is now on");
                        status = LightStatus::On;
                    }
                    LightMsg::Off => {
                        println!("Light is now off");
                        status = LightStatus::Off;
                    }
                    LightMsg::Disconnect => {
                        println!("Light is now off");
                        status = LightStatus::Off;
                        break;
                    }
                },
                Err(_) => {
                    println!("Error receiving message");
                    break;
                }
            }
        }
        status
    })
}

// Topic: Channels
//
// Summary:
//   Using the existing code, create a program that simulates an internet-of-things
//   remote control light bulb. The color of the light can be changed remotely.
//   Use threads and channels to communicate what color the light bulb should display.
//
// Requirements:
// * Create a separate thread representing the light bulb
// * Use a channel to communicate with the thread
// * Display a color change message using the println! macro
// * The light bulb must also be able to turn on and off
//   * Display whether the light is on or off on each color change
// * Turn off the light when disconnecting from it
//
// Notes:
// * Remember to add `crossbeam-channel` to your Cargo.toml file
// * Use the `colored` crate if you want to get fancy and display actual colors
// * The docs.rs site can be used to read documentation for third-party crates
// * Disconnection can be accomplished by dropping the sender, or
//   by telling the thread to self-terminate
// * Use `cargo test --bin a39` to test your program to ensure all cases are covered
