use super::super::ClipError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Content(String);

impl Content {
    pub fn new(content: String) -> Result<Self, ClipError> {
        if content.trim().is_empty() {
            return Err(ClipError::EmptyContent);
        }

        Ok(Self(content))
    }
    pub fn into_inner(self) -> String {
        self.0
    }
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}
