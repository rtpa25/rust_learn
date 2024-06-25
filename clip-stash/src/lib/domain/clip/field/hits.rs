use derive_more::Constructor;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Constructor)]
pub struct Hits(u64);

// the new function is implemented by the contructor derive macro
impl Hits {
    pub fn into_inner(self) -> u64 {
        self.0
    }
}
