/**
 * From/Into cannot fail
 * Almost always want to implemtn from for errors
 * Prefer implementing from instead of into
 * into is automatically implemented with from
 * user .into() when obvious what resulting type will be
 * Type::from() when important to knowt the resulting type
 *
 * From and Into functions always need to succed
 *
 * Fallible type converstion use when there is the possibility of failireu
 * just like from/into but return a Result
 */
use thiserror::Error;

enum Status {
    Broken(u8),
    Working,
}

impl From<u8> for Status {
    fn from(value: u8) -> Self {
        match value {
            0 => Status::Working,
            1..=255 => Status::Broken(value),
        }
    }
}

fn legacy_interface() -> u8 {
    5
}

fn main() {
    println!("Hello, world!");

    let status = Status::from(legacy_interface());
    let status: Status = legacy_interface().into();

    match NonZero::try_from(9) {
        Ok(non_zero) => println!("{} is non-zero", non_zero.0),
        Err(_) => println!("0 is zero"),
    }

    let whoops: Result<NonZero, _> = 0.try_into();
    match whoops {
        Ok(_) => println!("This should not happen"),
        Err(_) => println!("This should happen"),
    }

    let upper = Uppercase::from("hello".to_string());
    println!("{}", upper.0);

    let upper: Uppercase = "hello".to_string().into();
    println!("{}", upper.0);

    let upper: Uppercase = "hello".into();
    println!("{}", upper.0);

    let uppercase = Uppercase::from("lowercase");
    println!("{}", uppercase.0);

    let key_event_1 = KeyEvent {
        keycode: 9,
        state: KeyPress::Down,
    };
    let key_event_2 = KeyEvent {
        keycode: 8,
        state: KeyPress::Down,
    };

    let input_event: InputEvent = key_event_1.into();
    println!("{:?}", &input_event);
    let input_event = InputEvent::from(key_event_2);
    println!("{:?}", input_event);

    match do_api_operation() {
        Ok(_) => println!("API operation successful"),
        Err(e) => println!("API operation failed: {}", e),
    }

    let a = 15_u8;
    let b = 20_u16;

    let c: u32 = a.into();
}

enum NonZeroError {
    IsZero,
}

struct NonZero(i32);

impl TryFrom<i32> for NonZero {
    type Error = NonZeroError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value == 0 {
            Err(NonZeroError::IsZero)
        } else {
            Ok(NonZero(value))
        }
    }
}

struct Uppercase(String);

impl From<String> for Uppercase {
    fn from(value: String) -> Self {
        Uppercase(value.to_uppercase())
    }
}

impl From<&str> for Uppercase {
    fn from(value: &str) -> Self {
        Uppercase(value.to_uppercase())
    }
}

#[derive(Debug)]
enum KeyPress {
    Down,
    Up,
}

#[derive(Debug)]
struct KeyEvent {
    keycode: u16,
    state: KeyPress,
}

#[derive(Debug)]
enum InputEvent {
    key(u16, KeyPress),
    Mouse { x: i32, y: i32 },
}

impl From<KeyEvent> for InputEvent {
    fn from(value: KeyEvent) -> Self {
        InputEvent::key(value.keycode, value.state)
    }
}

#[derive(Error, Debug)]
enum NetworkError {
    #[error("connection timedout")]
    Timeout,
    #[error("connection refused")]
    ServerError,
}

#[derive(Error, Debug)]
enum DatabaseError {
    #[error("connection failed to database")]
    ConnectionError,
    #[error("query failed")]
    QueryError,
}

#[derive(Error, Debug)]
enum APIError {
    #[error("network error: {0}")]
    Network(#[from] NetworkError),
    #[error("database error: {0}")]
    Database(#[from] DatabaseError),
}

fn do_api_operation() -> Result<(), APIError> {
    Err(NetworkError::Timeout)?
}

// Topic: TryFrom/TryInto
//
// Summary:
// * A library is needed for an application to convert hex color codes
//   into their component color values (red, green, and blue). Hex color codes
//   consist of a hash symbol followed by six hex digits. Every two hex digits
//   represent a color component in the order of red, green, blue.
//
//   Example hex color codes:
//    #ffffff -> Rgb(255, 255, 255)
//    #001122 -> Rgb(0, 17, 34)
//
// Requirements:
// * Create a program to convert a hex code (as &str) into an Rgb struct
// * Implement TryFrom to perform the conversion
// * Utilize the question mark operator in your implementation
//
// Notes:
// * See the `from_str_radix` function in the stdlib docs for `u8`
//   to convert hex digits to `u8`
//   * Hex digits use a radix value of 16
// * Utilize the `thiserror` crate for your error type
// * Run `cargo test --bin a37` to test your implementation

#[derive(Debug, Eq, PartialEq)]
struct Rgb(u8, u8, u8);

#[derive(Error, Debug)]
enum RgbError {
    #[error("invalid hex digit")]
    ParseError(std::num::ParseIntError),
    #[error("missing hash symbol")]
    MissingHash,
    #[error("invalid hex color length")]
    LengthError,
}

impl From<std::num::ParseIntError> for RgbError {
    fn from(err: std::num::ParseIntError) -> Self {
        RgbError::ParseError(err)
    }
}

impl TryFrom<&str> for Rgb {
    type Error = RgbError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if !value.starts_with('#') {
            return Err(RgbError::MissingHash);
        }

        if value.len() != 7 {
            return Err(RgbError::LengthError);
        }

        let r = u8::from_str_radix(&value[1..3], 16)?;
        let g = u8::from_str_radix(&value[3..5], 16)?;
        let b = u8::from_str_radix(&value[5..7], 16)?;

        Ok(Self(r, g, b))
    }
}

#[cfg(test)]
mod test {
    use super::Rgb;
    use std::convert::TryFrom;

    #[test]
    fn converts_valid_hex_color() {
        let expected = Rgb(0, 204, 102);
        let actual = Rgb::try_from("#00cc66");
        assert_eq!(
            actual.is_ok(),
            true,
            "valid hex code should be converted to Rgb"
        );
        assert_eq!(actual.unwrap(), expected, "wrong Rgb value");
    }

    #[test]
    fn fails_on_invalid_hex_digits() {
        assert_eq!(
            Rgb::try_from("#0011yy").is_err(),
            true,
            "should be an error with invalid hex color"
        );
    }

    #[test]
    fn fails_when_missing_hash() {
        assert_eq!(
            Rgb::try_from("001100").is_err(),
            true,
            "should be an error when missing hash symbol"
        );
    }

    #[test]
    fn fails_when_missing_color_components() {
        assert_eq!(
            Rgb::try_from("#0011f").is_err(),
            true,
            "should be an error when missing one or more color components"
        );
    }

    #[test]
    fn fails_with_too_many_color_components() {
        assert_eq!(
            Rgb::try_from("#0011ffa").is_err(),
            true,
            "should be an error when too many color components are provided"
        );
    }
}
