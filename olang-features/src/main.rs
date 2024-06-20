/**
 * Sometimes the compiler cannot determins the type of some data
 * A few options are available when this happens:
 *  - Type Annotations
 *  - Turbofish -- item::<Type>
 *    - This is allowed on every function where we have a generic type
 */

/**
 * Break can return a value to a loop expression
 * Only valid on a loop (not while or for)
 * Can also break to a loop label with a value
 */

/**
 * Structs may have many fields to set during creation
 *  lots of code
 * Default can be used to set the default values
 *  Sometimes one or two fields may need to have non-default values
 *  Possible mutability, lots of boilerplate
 */

/**
 * Escape sequences
 * Not always possible or convienent to include certain charecters in a string
 * - Newline
 * - Tab
 * - Quote
 * - Unicode
 *
 * Escape sequences allow inclusion of these charecters
 */

/**
 * Raw strings
 * Sometimes escape sequences are not desired or disabled
 * Raw strings are prefixed with r#""#
 * No escape sequences are processed
 * Can be useful for regex, file paths, etc
 */

fn main() {
    let numbers = vec![34, 50, 25, 100, 65];

    let odds = numbers.iter().filter(|n| **n % 2 == 1).collect::<Vec<_>>();

    dbg!(odds);

    let even_matrix = [[2, 4, 6], [8, 10, 12], [14, 16, 18]];

    'rows: for row in even_matrix.iter() {
        '_cols: for col in row.iter() {
            if col % 2 == 1 {
                println!("Found an odd number: {}", col);
                break 'rows;
            }
            println!("Found an even number: {}", col)
        }
    }

    let value = 5_usize;

    let result = 'ident: loop {
        // both variations work
        break value;
        break 'ident value;
    };

    let particle = Particle {
        x: 10.0,
        y: 20.0,
        z: 30.0,
        ..Default::default()
    };

    dbg!(particle);

    let msg = "Hello\nworld!";
    println!("{}", msg);
    let msg = "Hello\tworld!";
    println!("{}", msg);
    let msg = "left\\right";
    println!("{}", msg);
    let msg = "Hello\u{1F600}world!";
    println!("{}", msg);
    let msg = "Over \"there\"";
    println!("{}", msg);

    let msg = r"Hello\nworld!";
    println!("{}", msg);

    let msg = r#"Hello     nworld!"#;
    println!("{}", msg);

    let msg = r###"Hello\nworld!"###;
    println!("{}", msg);

    let msg = r####""Hello"\nworl#####d!"####;
    println!("{}", msg);

    let msg = r"🙂";
    println!("{}", msg);
}

#[derive(Debug)]
struct Particle {
    x: f64,
    y: f64,
    z: f64,
    vx: f64,
    vy: f64,
    vz: f64,
}
impl Default for Particle {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            vx: 0.0,
            vy: 0.0,
            vz: 0.0,
        }
    }
}
