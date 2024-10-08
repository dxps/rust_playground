use crate::{data::*, error, error::Error::*, DbConn, DbPool};
use mobc_postgres::{tokio_postgres, PgConnectionManager};
use tokio_postgres::{Config, Error, NoTls, Row};

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
    let config = Config::from_str("postgres://postgres:postgres@127.0.0.1:7999/postgres")?;

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

pub async fn create_todo(db_pool: &DbPool, body: TodoRequest) -> Result<Todo> {
    let conn = get_db_conn(db_pool).await?;
    let query = format!("INSERT INTO {} (name) VALUES ($1) RETURNING *", TABLE);
    let row = conn
        .query_one(query.as_str(), &[&body.name])
        .await
        .map_err(DbQueryError)?;
    Ok(row_to_todo(&row))
}

fn row_to_todo(row: &Row) -> Todo {
    let id: i32 = row.get(0);
    let name: String = row.get(1);
    let checked: bool = row.get(3);
    Todo { id, name, checked }
}

pub async fn fetch_todos(db_pool: &DbPool, search: Option<String>) -> Result<Vec<Todo>> {
    let con = get_db_conn(db_pool).await?;
    let where_clause = match search {
        Some(_) => "WHERE name like $1",
        None => "",
    };
    let query = format!(
        "SELECT {} FROM {} {} ORDER BY created_at DESC",
        SELECT_FIELDS, TABLE, where_clause
    );
    let q = match search {
        Some(v) => con.query(query.as_str(), &[&v]).await,
        None => con.query(query.as_str(), &[]).await,
    };
    let rows = q.map_err(DbQueryError)?;

    Ok(rows.iter().map(|r| row_to_todo(&r)).collect())
}

pub async fn update_todo(db_pool: &DbPool, id: i32, body: TodoUpdateRequest) -> Result<Todo> {
    let con = get_db_conn(db_pool).await?;
    let query = format!(
        "UPDATE {} SET name = $1, checked = $2 WHERE id = $3 RETURNING *",
        TABLE
    );
    let row = con
        .query_one(query.as_str(), &[&body.name, &body.checked, &id])
        .await
        .map_err(DbQueryError)?;
    Ok(row_to_todo(&row))
}

pub async fn delete_todo(db_pool: &DbPool, id: i32) -> Result<u64> {
    let con = get_db_conn(db_pool).await?;
    let query = format!("DELETE FROM {} WHERE id = $1", TABLE);
    con.execute(query.as_str(), &[&id])
        .await
        .map_err(DbQueryError)
}
