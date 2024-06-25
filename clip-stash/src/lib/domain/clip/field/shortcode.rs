use std::str::FromStr;

use derive_more::From;
use serde::{Deserialize, Serialize};

use crate::domain::clip::ClipError;

#[derive(Debug, Clone, Serialize, Deserialize, From)]
pub struct Shortcode(String);

impl Shortcode {
    pub fn new() -> Self {
        use rand::prelude::*;
        let allowed_chars: Vec<_> =
            "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"
                .chars()
                .collect();

        let mut rng = thread_rng();
        let mut short_code = String::with_capacity(10);

        for _ in 0..10 {
            let idx = rng.gen_range(0..allowed_chars.len());
            short_code.push(allowed_chars[idx]);
        }

        Self(short_code)
    }
    pub fn into_inner(self) -> String {
        self.0
    }
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl Default for Shortcode {
    fn default() -> Self {
        Self::new()
    }
}

impl From<Shortcode> for String {
    fn from(shortcode: Shortcode) -> Self {
        shortcode.into_inner()
    }
}

impl From<&str> for Shortcode {
    fn from(shortcode: &str) -> Self {
        Shortcode(shortcode.into())
    }
}

impl FromStr for Shortcode {
    type Err = ClipError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.into()))
    }
}
