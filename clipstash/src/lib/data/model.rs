use crate::ClipError;
use crate::{data::DbId, Time};
use chrono::NaiveDateTime;
use std::convert::TryFrom;

#[derive(Debug, sqlx::FromRow)]
pub struct Clip {
    pub(in crate::data) clip_id: String,
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

    fn try_from(clip: Clip) -> Result<Self, Self::Error> {
        use crate::domain::clip::field;
        use std::str::FromStr;
        Ok(Self {
            clip_id: field::ClipId::new(DbId::from_str(clip.clip_id.as_str())?),
            shortcode: field::ShortCode::from(clip.shortcode),
            content: field::Content::new(clip.content.as_str())?,
            title: field::Title::new(clip.title),
            posted: field::Posted::new(Time::from_naive_utc(clip.posted)),
            expires: field::Expires::new(clip.expires.map(Time::from_naive_utc)),
            password: field::Password::new(clip.password.unwrap_or_default())?,
            hits: field::Hits::new(u64::try_from(clip.hits)?),
        })
    }
}
