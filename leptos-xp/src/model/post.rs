use serde::{Deserialize, Serialize};

#[cfg(feature = "hydrate")]
#[cfg(not(feature = "ssr"))]
use chrono::{Local, NaiveDateTime};

#[cfg(feature = "ssr")]
use sqlx::types::chrono::{Local, NaiveDateTime};
#[cfg(feature = "ssr")]
use sqlx::FromRow;

//#[cfg_attr(feature = "hydrate", derive(Serialize, Deserialize, Debug, Clone))]
// #[cfg_attr(feature = "ssr", derive(Serialize, Deserialize, Debug, Clone, FromRow))]
#[cfg_attr(feature = "ssr", derive(FromRow))]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Post {
    pub id: String,
    pub dt: NaiveDateTime,
    pub image_url: String,
    pub title: String,
    pub content: String,
}

impl Post {
    pub fn new_empty() -> Self {
        Self {
            id: "".into(),
            dt: Local::now().naive_local(),
            image_url: "".into(),
            title: "".into(),
            content: "".into(),
        }
    }
}
