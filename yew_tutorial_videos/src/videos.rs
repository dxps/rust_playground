use serde::Deserialize;

#[derive(Clone, Deserialize, PartialEq)]
pub struct Video {
    pub id: usize,
    pub title: String,
    pub speaker: String,
    pub url: String,
}
