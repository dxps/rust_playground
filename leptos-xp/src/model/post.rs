use chrono::{Local, NaiveDateTime};
use serde::{Deserialize, Serialize};

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
