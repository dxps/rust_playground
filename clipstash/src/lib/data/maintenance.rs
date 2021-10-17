use super::{DatabasePool, Result};

pub async fn delete_expired(pool: &DatabasePool) -> Result<u64> {
    Ok(
        sqlx::query!(r#"DELETE FROM clips WHERE strftime('%s', 'now') > expires"#)
            .execute(pool)
            .await?
            .rows_affected(),
    )
}
