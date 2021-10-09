use crate::{data, service, ShortCode};

use super::{DatabasePool, Result};

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

pub async fn get_clip<M: Into<data::GetClip>>(model: M, pool: &DatabasePool) -> Result<data::Clip> {
    let model = model.into();
    let shortcode = model.shortcode.as_str();
    Ok(sqlx::query_as!(
        data::Clip,
        "SELECT * FROM clips WHERE shortcode = ?",
        shortcode
    )
    .fetch_one(pool)
    .await?)
}
