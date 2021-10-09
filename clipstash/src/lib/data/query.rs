use chrono::Utc;

use super::{clip, DbId};
use crate::{
    data,
    data::{DataError, DatabasePool},
    service, ShortCode,
};

/// A type alias for a result that might contain `DataError`.
type Result<T> = std::result::Result<T, DataError>;

// ----------------
// The Query Models
// ----------------

pub struct GetClip {
    pub(in crate::data) shortcode: String,
}

impl From<ShortCode> for GetClip {
    fn from(shortcode: ShortCode) -> Self {
        GetClip {
            shortcode: shortcode.into_inner(),
        }
    }
}

impl From<String> for GetClip {
    fn from(shortcode: String) -> Self {
        GetClip { shortcode }
    }
}

impl From<service::GetClip> for GetClip {
    fn from(req: service::GetClip) -> Self {
        Self {
            shortcode: req.shortcode.into_inner(),
        }
    }
}

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

pub struct UpdateClip {
    pub(in crate::data) shortcode: String,
    pub(in crate::data) content: String,
    pub(in crate::data) title: Option<String>,
    pub(in crate::data) expires: Option<i64>,
    pub(in crate::data) password: Option<String>,
}

impl From<service::UpdateClip> for UpdateClip {
    fn from(req: service::UpdateClip) -> Self {
        Self {
            shortcode: req.shortcode.into_inner(),
            content: req.content.into_inner(),
            title: req.title.into_inner(),
            expires: req.expires.into_inner().map(|time| time.timestamp()),
            password: req.password.into_inner(),
        }
    }
}

// ---------------
// The Query Logic
// ---------------

pub async fn get_clip<M: Into<GetClip>>(model: M, pool: &DatabasePool) -> Result<data::Clip> {
    let model = model.into();
    let shortcode = model.shortcode.as_str();
    Ok(sqlx::query_as!(
        clip::Clip,
        "SELECT * FROM clips WHERE shortcode = ?",
        shortcode
    )
    .fetch_one(pool)
    .await?)
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

pub async fn update_clip<M: Into<UpdateClip>>(model: M, pool: &DatabasePool) -> Result<data::Clip> {
    let model = model.into();
    let _ = sqlx::query!(
        r#"UPDATE clips SET content = ?, title = ?, expires = ?, password = ?
            WHERE shortcode = ?"#,
        model.content,
        model.title,
        model.expires,
        model.password,
        model.shortcode
    )
    .execute(pool)
    .await?;
    get_clip(model.shortcode, pool).await
}
