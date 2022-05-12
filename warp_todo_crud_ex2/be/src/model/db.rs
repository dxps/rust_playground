use std::time::Duration;

use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

const PG_HOST: &str = "localhost:5439";
const PG_DB: &str = "test";
const PG_USER: &str = "test";
const PG_PASS: &str = "test";
const PG_MAX_CONNS: u32 = 3;

pub type DbPool = Pool<Postgres>;

pub async fn init_db() -> Result<DbPool, sqlx::Error> {
    new_db_pool(PG_HOST, PG_DB, PG_USER, PG_PASS, PG_MAX_CONNS).await
}

async fn new_db_pool(
    host: &str,
    db: &str,
    user: &str,
    pwd: &str,
    max_conns: u32,
) -> Result<DbPool, sqlx::Error> {
    let conn_str = format!("postgres://{}:{}@{}/{}", user, pwd, host, db);
    PgPoolOptions::new()
        .max_connections(max_conns)
        .connect_timeout(Duration::from_millis(500))
        .connect(&conn_str)
        .await
}

#[cfg(test)]
#[path = "../zz_tests/model_db.rs"]
mod tests;
