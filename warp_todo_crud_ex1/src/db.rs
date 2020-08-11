use crate::{error, error::Error::*, DbConn, DbPool};
use mobc_postgres::{tokio_postgres, PgConnectionManager};
use tokio_postgres::{Config, Error, NoTls};

use std::str::FromStr;
use std::time::Duration;

const DB_POOL_MAX_OPEN: u64 = 32;
const DB_POOL_MAX_IDLE: u64 = 2;
const DB_POOL_TIMEOUT_SECONDS: u64 = 10;

const INIT_SQL_FILE: &str = "./db.sql";
const TABLE: &str = "todo";
const SELECT_FIELDS: &str = "id, name, created_at, checked";

/// A convenience type that includes one of this app's errors in case of an failure.
type Result<T> = std::result::Result<T, error::Error>;

/// Initialize the database.
pub async fn init_db(db_pool: &DbPool) -> Result<()> {
    let init_sql = std::fs::read_to_string(INIT_SQL_FILE)?;
    let conn = get_db_conn(db_pool).await?;
    conn.batch_execute(init_sql.as_str())
        .await
        .map_err(DbInitError)?;
    Ok(())
}

/// It creates a database connection pool.
/// Note that no connection to the database is created.
pub fn create_pool() -> std::result::Result<DbPool, mobc::Error<Error>> {
    let config = Config::from_str("postgres://postgres@127.0.0.1:7999/postgres")?;

    let manager = PgConnectionManager::new(config, NoTls);
    Ok(DbPool::builder()
        .max_open(DB_POOL_MAX_OPEN)
        .max_idle(DB_POOL_MAX_IDLE)
        .get_timeout(Some(Duration::from_secs(DB_POOL_TIMEOUT_SECONDS)))
        .build(manager))
}

/// Get a (new or existing) database connection from the pool.
pub async fn get_db_conn(db_pool: &DbPool) -> Result<DbConn> {
    db_pool.get().await.map_err(DbPoolError)
}
