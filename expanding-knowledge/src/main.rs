use std::vec;

#[derive(Debug, Clone, Copy)]
enum Position {
    Manager,
    Supervisor,
    Worker,
}

#[derive(Debug, Clone, Copy)]
struct Employee {
    work_hours: u32,
    position: Position,
}

enum Mouse {
    LeftClick,
    RightClick,
    MiddleClick,
    Scroll(i32),
    Move(i32, i32),
}

#[derive(Debug)]
enum PromoDiscount {
    NewUser,
    Holiday(String),
}
#[derive(Debug)]
enum Discount {
    Percent(f32),
    Flat(f32),
    Promo(PromoDiscount),
    Custom(String),
}

struct Ticket {
    event: String,
    price: i32,
}

fn print_ticket(ticket: Ticket) {
    let n = 3;
    match n {
        3 => println!("3"),
        other => println!("number: {}", other),
    }

    let flat_discount = Discount::Flat(10.0);
    match flat_discount {
        Discount::Flat(amount) => println!("Flat discount: {}", amount),
        other => println!("other discount: {:?}", other),
    }

    let ticket = Ticket {
        event: "Concert".to_string(),
        price: 50,
    };

    match ticket {
        Ticket { price: 50, .. } => println!("Standard ticket"),
        Ticket { price, .. } if price > 50 => println!("Expensive ticket"),
        Ticket { price, .. } if price < 50 => println!("Cheap ticket"),
        Ticket { event, .. } => println!("Event: {}", event),
    }
}

fn print_employee(employee: Employee) {
    println!("{:?}", employee);
}

fn main() {
    let me = Employee {
        work_hours: 40,
        position: Position::Manager,
    };

    let me = Employee {
        position: Position::Supervisor,
        ..me
    };

    println!("{:?}", me);
    // automatically creates a copy of the struct and the actual struct is not moved
    print_employee(me);
    print_employee(me);

    let discount = Discount::Promo(PromoDiscount::Holiday("Christmas".to_string()));
    println!("{:?}", discount);

    let scores: Vec<i32>;

    scores = "1,2,3,4,5"
        .split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    ran();
    survey();

    let numbers = vec![1, 2, 3, 4, 5];
    match numbers.is_empty() {
        true => println!("No numbers"),
        false => println!("Numbers"),
    }
}

enum CTicket {
    Backstage(String, u32),
    Standard(u32),
    VIP(String, u32),
}
fn ran() {
    let t1 = CTicket::Standard(50);
    let t2 = CTicket::Backstage("Ronit".to_string(), 100);
    let t3 = CTicket::VIP("Vans".to_string(), 75);

    let tickets = vec![t1, t2, t3];

    for ticket in tickets {
        match ticket {
            CTicket::Standard(price) => println!("Standard ticket: {}", price),
            CTicket::Backstage(name, price) => println!("Backstage ticket: {} {}", name, price),
            CTicket::VIP(name, price) => println!("VIP ticket: {} {}", name, price),
        }
    }
}

struct Survey {
    q1: Option<i32>,
    q2: Option<bool>,
    q3: Option<String>,
}

fn survey() {
    let survey = Survey {
        q1: Some(1),
        q2: None,
        q3: Some("Hello".to_string()),
    };

    match survey.q1 {
        Some(answer) => println!("Q1: {}", answer),
        None => println!("Q1: No answer"),
    }

    match survey.q2 {
        Some(answer) => println!("Q2: {}", answer),
        None => println!("Q2: No answer"),
    }

    match survey.q3 {
        Some(answer) => println!("Q3: {}", answer),
        None => println!("Q3: No answer"),
    }
}

struct StudentLockers {
    name: String,
    marks: Option<i32>,
}

impl StudentLockers {
    fn new(name: &str) -> Self {
        StudentLockers {
            name: name.to_string(),
            marks: None,
        }
    }

    fn set_marks(&mut self, marks: i32) {
        self.marks = Some(marks);
    }

    fn get_marks(&self) -> i32 {
        self.marks.unwrap_or(0)
    }

    fn print_marks(&self) {
        match self.marks {
            Some(marks) => println!("{}: {}", self.name, marks),
            None => println!("{}: No marks", self.name),
        }
    }
}

/// A color enum
enum Color {
    Red,
    Blue,
}

/// A ball
struct Ball {
    /// The color of the ball
    color: Color,
    /// The size of the ball
    size: u32,
}

/// A ball function
fn ball() {
    let ball = Ball {
        color: Color::Red,
        size: 10,
    };

    match ball.color {
        Color::Red => println!("Red ball"),
        Color::Blue => println!("Blue ball"),
    }
}

fn print_string_cases(s: &str) {
    println!("this is uppercased: {}", s.to_uppercase());
    println!("this is lowercased: {}", s.to_lowercase());
}
