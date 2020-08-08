use crate::{DbConn, DbPool};
use mobc_postgres::{tokio_postgres, PgConnectionManager};
use tokio_postgres::{Config, Error, NoTls};

use std::fs;
use std::str::FromStr;
use std::time::Duration;

const DB_POOL_MAX_OPEN: u64 = 32;
const DB_POOL_MAX_IDLE: u64 = 2;
const DB_POOL_TIMEOUT_SECONDS: u64 = 10;

const INIT_SQL: &str = "./db.sql";

/// It creates a database connection pool. Note that no database connection is created.
pub fn create_pool() -> std::result::Result<DbPool, mobc::Error<Error>> {
    let config = Config::from_str("postgres://postgres@127.0.0.1:7999/postgres")?;

    let manager = PgConnectionManager::new(config, NoTls);
    Ok(DbPool::builder()
        .max_open(DB_POOL_MAX_OPEN)
        .max_idle(DB_POOL_MAX_IDLE)
        .get_timeout(Some(Duration::from_secs(DB_POOL_TIMEOUT_SECONDS)))
        .build(manager))
}

///
pub async fn get_db_conn(db_pool: &DbPool) -> Result<DbConn> {
    db_pool.get().await.map_err(DBPoolError)
}
