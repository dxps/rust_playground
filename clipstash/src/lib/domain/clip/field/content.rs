use crate::domain::clip::ClipError;
use rocket::form::{self, FromFormField, ValueField};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Content(String);

impl Content {
    pub fn new(s: &str) -> Result<Self, ClipError> {
        if !s.trim().is_empty() {
            Ok(Self(s.to_owned()))
        } else {
            Err(ClipError::EmptyContent)
        }
    }

    pub fn into_inner(self) -> String {
        self.0
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[rocket::async_trait]
impl<'r> FromFormField<'r> for Content {
    fn from_value(field: ValueField<'r>) -> rocket::form::Result<'r, Self> {
        Ok(Self::new(field.value).map_err(|err| form::Error::validation(format!("{}", err)))?)
    }
}
