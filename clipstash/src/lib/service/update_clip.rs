use std::convert::TryInto;

use crate::{
    data::{self, DatabasePool},
    domain::clip::field,
    Clip, ServiceError,
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateClip {
    pub shortcode: field::ShortCode,
    pub content: field::Content,
    pub title: field::Title,
    pub expires: field::Expires,
    pub password: field::Password,
}

pub async fn update_clip(req: UpdateClip, pool: &DatabasePool) -> Result<Clip, ServiceError> {
    Ok(data::update_clip(req, pool).await?.try_into()?)
}
