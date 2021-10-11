use crate::ServiceError;

use super::{DatabasePool, Transaction};

pub async fn begin_transaction(pool: &DatabasePool) -> Result<Transaction<'_>, ServiceError> {
    Ok(pool.begin().await?)
}

pub async fn commit_transaction(txn: Transaction<'_>) -> Result<(), ServiceError> {
    Ok(txn.commit().await?)
}
