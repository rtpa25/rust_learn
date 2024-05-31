use chrono::prelude::*;
use humantime::format_duration;
use std::time::Duration;

mod greetings {
    pub fn good_bye() {
        println!("Good bye!");
    }

    pub fn hello() {
        println!("Hello, world!");
    }
}

mod math {
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    pub fn sub(a: i32, b: i32) -> i32 {
        a - b
    }
}

fn main() {
    greetings::hello();
    greetings::good_bye();

    let a = 10;
    let b = 20;

    println!("{} + {} = {}", a, b, math::add(a, b));
    println!("{} - {} = {}", a, b, math::sub(a, b));

    let d = Duration::from_secs(1234);
    println!("Duration: {}", format_duration(d));

    let local_time = Local::now();
    println!(
        "Local time: {}",
        local_time.format("%Y-%m-%d %H:%M:%S").to_string()
    );
}

fn all_caps(workd: &str) -> String {
    workd.to_uppercase()
}

#[cfg(test)]
mod test {
    use crate::all_caps;

    #[test]
    fn check_all_caps() {
        let result = all_caps("hello");
        let expected = String::from("HELLO");
        assert_eq!(result, expected, "string should be in all caps");
    }
}
