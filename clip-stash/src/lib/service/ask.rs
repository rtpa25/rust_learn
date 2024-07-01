use crate::domain::clip::field;
use crate::Shortcode;

use derive_more::Constructor;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Constructor)]
pub struct GetClip {
    pub shortcode: Shortcode,
    pub password: field::Password,
}

impl GetClip {
    pub fn from_raw(shortcode: &str) -> Self {
        Self {
            shortcode: Shortcode::from(shortcode),
            password: field::Password::default(),
        }
    }
}

impl From<Shortcode> for GetClip {
    fn from(value: Shortcode) -> Self {
        Self {
            shortcode: value,
            password: field::Password::default(),
        }
    }
}

impl From<&str> for GetClip {
    fn from(value: &str) -> Self {
        Self::from_raw(value)
    }
}

#[derive(Debug, Serialize, Deserialize, Constructor)]
pub struct NewClip {
    pub content: field::Content,
    pub title: field::Title,
    pub expires: field::Expires,
    pub password: field::Password,
}

#[derive(Debug, Serialize, Deserialize, Constructor)]
pub struct UpdateClip {
    pub content: field::Content,
    pub title: field::Title,
    pub expires: field::Expires,
    pub password: field::Password,
    pub shortcode: Shortcode,
}
