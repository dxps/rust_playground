use crate::{
    data,
    data::{get_clip, DatabasePool},
    service,
};

use super::Result;

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
