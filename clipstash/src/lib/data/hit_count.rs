use crate::ShortCode;

use super::{DatabasePool, Result};

pub async fn increase_hit_count(
    shortcode: &ShortCode,
    hits: u32,
    pool: &DatabasePool,
) -> Result<()> {
    let shortcode = shortcode.as_str();
    Ok(sqlx::query!(
        "UPDATE clips SET hits = hits + ? WHERE shortcode  = ?",
        hits,
        shortcode
    )
    .execute(pool)
    .await
    .map(|_| ())?)
}
