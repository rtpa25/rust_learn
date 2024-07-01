use crate::data::DbId;
use crate::{ClipError, Shortcode, Time};
use chrono::{NaiveDateTime, Utc};
use std::convert::TryFrom;
use std::str::FromStr;

#[derive(Debug, sqlx::FromRow)]
pub struct Clip {
    pub(in crate::data) clip_id: String, // this is public only in the crate data module
    pub(in crate::data) shortcode: String,
    pub(in crate::data) content: String,
    pub(in crate::data) title: Option<String>,
    pub(in crate::data) posted: NaiveDateTime,
    pub(in crate::data) expires: Option<NaiveDateTime>,
    pub(in crate::data) password: Option<String>,
    pub(in crate::data) hits: i64,
}

impl TryFrom<Clip> for crate::domain::Clip {
    type Error = ClipError;
    fn try_from(value: Clip) -> Result<Self, Self::Error> {
        use crate::domain::clip::field;
        Ok(Self {
            clip_id: field::ClipId::new(DbId::from_str(value.clip_id.as_str())?),
            shortcode: field::Shortcode::from_str(value.shortcode.as_str())?,
            content: field::Content::new(value.content)?,
            title: field::Title::new(value.title),
            posted: field::Posted::new(Time::from_naive_utc(value.posted)),
            expires: field::Expires::new(value.expires.map(Time::from_naive_utc)),
            password: field::Password::new(value.password)?,
            hits: field::Hits::new(u64::try_from(value.hits)?),
        })
    }
}

pub struct GetClip {
    pub(in crate::data) shortcode: String,
}

impl From<crate::service::ask::GetClip> for GetClip {
    fn from(value: crate::service::ask::GetClip) -> Self {
        Self {
            shortcode: value.shortcode.into_inner(),
        }
    }
}

impl From<Shortcode> for GetClip {
    fn from(value: Shortcode) -> Self {
        Self {
            shortcode: value.into_inner(),
        }
    }
}

impl From<String> for GetClip {
    fn from(value: String) -> Self {
        Self { shortcode: value }
    }
}

pub struct NewClip {
    pub(in crate::data) clip_id: String,
    pub(in crate::data) shortcode: String,
    pub(in crate::data) content: String,
    pub(in crate::data) title: Option<String>,
    pub(in crate::data) posted: i64,
    pub(in crate::data) expires: Option<i64>,
    pub(in crate::data) password: Option<String>,
}

impl From<crate::service::ask::NewClip> for NewClip {
    fn from(value: crate::service::ask::NewClip) -> Self {
        Self {
            clip_id: DbId::new().to_string(),
            shortcode: Shortcode::default().into(),
            content: value.content.into_inner(),
            title: value.title.into_inner(),
            posted: Utc::now().timestamp(),
            expires: value.expires.into_inner().map(|e| e.timestamp()),
            password: value.password.into_inner(),
        }
    }
}

pub struct UpdateClip {
    pub(in crate::data) shortcode: String,
    pub(in crate::data) content: String,
    pub(in crate::data) title: Option<String>,
    pub(in crate::data) expires: Option<i64>,
    pub(in crate::data) password: Option<String>,
}

impl From<crate::service::ask::UpdateClip> for UpdateClip {
    fn from(value: crate::service::ask::UpdateClip) -> Self {
        Self {
            shortcode: value.shortcode.into_inner(),
            content: value.content.into_inner(),
            title: value.title.into_inner(),
            expires: value.expires.into_inner().map(|e| e.timestamp()),
            password: value.password.into_inner(),
        }
    }
}
