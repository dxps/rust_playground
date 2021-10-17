use crate::{data, data::DatabasePool, ServiceError};

pub async fn delete_expired(pool: &DatabasePool) -> Result<u64, ServiceError> {
    Ok(data::maintenance::delete_expired(pool).await?)
}
