use std::convert::TryInto;

use serde::{Deserialize, Serialize};

use crate::{
    data::{self, DatabasePool},
    domain::clip::field::Password,
    Clip, ServiceError, ShortCode,
};

#[derive(Debug, Deserialize, Serialize)]
pub struct GetClip {
    pub shortcode: ShortCode,
    pub password: Password,
}

impl GetClip {
    pub fn from_raw(shortcode: &str) -> Self {
        Self {
            shortcode: ShortCode::from(shortcode),
            password: Password::default(),
        }
    }
}

impl From<ShortCode> for GetClip {
    fn from(shortcode: ShortCode) -> Self {
        Self {
            shortcode,
            password: Password::default(),
        }
    }
}

impl From<&str> for GetClip {
    fn from(raw: &str) -> Self {
        Self::from_raw(raw)
    }
}

pub async fn get_clip(req: GetClip, pool: &DatabasePool) -> Result<Clip, ServiceError> {
    let password = req.password.clone();
    let clip: Clip = data::get_clip(req, pool).await?.try_into()?;
    if clip.password.has_password() {
        if clip.password == password {
            Ok(clip)
        } else {
            Err(ServiceError::PermissionError("Invalid password".to_owned()))
        }
    } else {
        Ok(clip)
    }
}
