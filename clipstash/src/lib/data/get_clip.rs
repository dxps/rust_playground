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

#[cfg(test)]
pub mod test {

    use crate::{
        data::{self, database::test::new_db, DbId, NewClip},
        test::async_runtime,
    };

    // Testing utility functions.

    fn data_new_clip(shortcode: &str) -> NewClip {
        use chrono::Utc;
        NewClip {
            clip_id: DbId::new().into(),
            content: format!("content for clip '{}'", shortcode),
            title: None,
            shortcode: shortcode.into(),
            posted: Utc::now().timestamp(),
            expires: None,
            password: None,
        }
    }

    #[test]
    fn clip_new_and_get() {
        let rt = async_runtime();
        let db = new_db(rt.handle());
        let pool = db.get_pool();
        // FYI: While using `data::new_clip()` we are also testing the `data::get_clip()`
        // that is done automatically under the hood.
        let clip =
            rt.block_on(async move { data::new_clip(data_new_clip("1"), &pool.clone()).await });
        assert!(clip.is_ok());
        let clip = clip.unwrap();
        assert_eq!(clip.shortcode, "1");
        assert_eq!(clip.content, "content for clip '1'");
    }
}
