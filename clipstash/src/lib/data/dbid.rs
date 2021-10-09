use derive_more::{Display, From};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use uuid::Uuid;

/// A general purpose internal database ID.
#[derive(Clone, Debug, From, Display, Deserialize, Serialize)]
pub struct DbId(Uuid);

impl DbId {
    pub fn new() -> DbId {
        Uuid::new_v4().into()
    }

    pub fn nil() -> DbId {
        Self(Uuid::nil())
    }
}

impl Default for DbId {
    fn default() -> Self {
        Self::new()
    }
}

impl FromStr for DbId {
    type Err = uuid::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(DbId(Uuid::parse_str(s)?))
    }
}

impl From<DbId> for String {
    fn from(dbid: DbId) -> Self {
        format!("{}", dbid.0)
    }
}
