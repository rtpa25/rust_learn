use crate::data::DbId;
use derive_more::Constructor;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Constructor)]
pub struct ClipId(DbId);

impl ClipId {
    pub fn into_inner(self) -> DbId {
        self.0
    }
}

impl From<DbId> for ClipId {
    fn from(value: DbId) -> Self {
        Self(value)
    }
}

impl Default for ClipId {
    fn default() -> Self {
        Self(DbId::nil())
    }
}
