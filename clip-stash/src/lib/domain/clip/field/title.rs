use std::str::FromStr;

use super::super::ClipError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Title(Option<String>);

impl Title {
    pub fn new<T: Into<Option<String>>>(title: T) -> Self {
        let title = title.into();
        match title {
            Some(title) => {
                if title.trim().is_empty() {
                    Self(None)
                } else {
                    Self(Some(title))
                }
            }
            None => Self(None),
        }
    }
    pub fn into_inner(self) -> Option<String> {
        self.0
    }
    pub fn as_str(&self) -> Option<&str> {
        self.0.as_deref()
    }
    pub fn has_title(&self) -> bool {
        self.0.is_some()
    }
}

impl Default for Title {
    fn default() -> Self {
        Self::new(None)
    }
}

impl FromStr for Title {
    type Err = ClipError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(s.to_string()))
    }
}
