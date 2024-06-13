#[derive(Debug, Clone, Copy)]
pub struct NeverZero(pub i32);

impl NeverZero {
    pub fn new(value: i32) -> Result<Self, String> {
        if value == 0 {
            Err("Zero is not allowed".to_string())
        } else {
            Ok(Self(value))
        }
    }
}
