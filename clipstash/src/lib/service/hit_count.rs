use crate::{
    data::{self, DatabasePool},
    ServiceError, ShortCode,
};

pub async fn increase_hit_count(
    shortcode: &ShortCode,
    hits: u32,
    pool: &DatabasePool,
) -> Result<(), ServiceError> {
    Ok(data::increase_hit_count(shortcode, hits, pool).await?)
}
