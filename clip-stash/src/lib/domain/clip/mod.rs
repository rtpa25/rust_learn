use serde::{Deserialize, Serialize};
use thiserror::Error;

pub mod field;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Clip {
    pub clip_id: field::ClipId,
    pub shortcode: field::Shortcode,
    pub content: field::Content,
    pub title: field::Title,
    pub posted: field::Posted,
    pub expires: field::Expires,
    pub password: field::Password,
    pub hits: field::Hits,
}

#[derive(Debug, Error)]
pub enum ClipError {
    #[error("Invalid password: {0}")]
    InvalidPassword(String),

    #[error("Invalid title: {0}")]
    InvalidTitle(String),

    #[error("Empty content")]
    EmptyContent,

    #[error("Invalid date: {0}")]
    InvalidDate(String),

    #[error("Date parse error: {0}")]
    DateParse(#[from] chrono::ParseError),

    #[error("Id parse error: {0}")]
    Id(#[from] uuid::Error),

    #[error("hits parse error: {0}")]
    Hits(#[from] std::num::TryFromIntError),
}
