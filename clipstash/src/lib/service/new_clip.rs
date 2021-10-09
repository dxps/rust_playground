use std::convert::TryInto;

use crate::{
    data::{self, DatabasePool},
    domain::clip::field,
    service, Clip, ServiceError,
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct NewClip {
    pub content: field::Content,
    pub title: field::Title,
    pub expires: field::Expires,
    pub password: field::Password,
}

pub async fn new_clip(req: service::NewClip, pool: &DatabasePool) -> Result<Clip, ServiceError> {
    Ok(data::new_clip(req, pool).await?.try_into()?)
}
