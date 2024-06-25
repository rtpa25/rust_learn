use chrono::{DateTime, NaiveDateTime, Utc};
use derive_more::From;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, From)]
pub struct Time(DateTime<Utc>);

impl Time {
    pub fn into_inner(self) -> DateTime<Utc> {
        self.0
    }
    pub fn timestamp(&self) -> i64 {
        self.0.timestamp()
    }
    pub fn from_naive_utc(naive_utc: NaiveDateTime) -> Self {
        Time(DateTime::from_naive_utc_and_offset(naive_utc, Utc))
    }
}

impl FromStr for Time {
    type Err = chrono::ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // The format is expected to be "YYYY-MM-DD"
        match format!("{}T00:00:00Z", s).parse::<DateTime<Utc>>() {
            Ok(time) => Ok(Time(time)),
            Err(e) => Err(e),
        }
    }
}
