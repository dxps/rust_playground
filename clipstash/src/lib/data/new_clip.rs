use chrono::Utc;

use crate::{
    data::{self, get_clip},
    service, ShortCode,
};

use super::{DatabasePool, DbId, Result};

pub struct NewClip {
    pub(in crate::data) clip_id: String,
    pub(in crate::data) shortcode: String,
    pub(in crate::data) content: String,
    pub(in crate::data) title: Option<String>,
    pub(in crate::data) posted: i64,
    pub(in crate::data) expires: Option<i64>,
    pub(in crate::data) password: Option<String>,
}

impl From<service::NewClip> for NewClip {
    fn from(req: service::NewClip) -> Self {
        Self {
            clip_id: DbId::new().into(),
            shortcode: ShortCode::default().into(),
            content: req.content.into_inner(),
            title: req.title.into_inner(),
            expires: req.expires.into_inner().map(|time| time.timestamp()),
            password: req.password.into_inner(),
            posted: Utc::now().timestamp(),
        }
    }
}

pub async fn new_clip<M: Into<NewClip>>(model: M, pool: &DatabasePool) -> Result<data::Clip> {
    let model = model.into();
    let _ = sqlx::query!(
        r#"INSERT INTO clips (
              clip_id, shortcode, content, title, posted, expires, password, hits)
           VALUES ( ?, ?, ?, ?, ?, ?, ?, ?)
        "#,
        model.clip_id,
        model.shortcode,
        model.content,
        model.title,
        model.posted,
        model.expires,
        model.password,
        0
    )
    .execute(pool)
    .await?;
    get_clip(model.shortcode, pool).await
}
