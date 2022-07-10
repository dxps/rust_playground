use std::{fs, path::PathBuf, time::Duration};

use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions},
    ConnectOptions, Pool, Postgres,
};

const PG_HOST: &str = "localhost";
// Root (user) specifics
const PG_ROOT_DB: &str = "postgres";
const PG_ROOT_USER: &str = "postgres";
const PG_ROOT_PWD: &str = "postgres";
// App specifics
const PG_DB: &str = "todo_app";
const PG_USER: &str = "todo_app";
const PG_PASS: &str = "todo_app";
const PG_MAX_CONNS: u32 = 3;

const SQL_DIR: &str = "ops/db_migration";
const SQL_RECREATE: &str = "00_recreate_db.sql";

pub type DbPool = Pool<Postgres>;

pub async fn init_db() -> Result<DbPool, sqlx::Error> {
    // DEV Only: Recreating the DB (drop & create, as root user).
    let root_db = new_db_pool(PG_HOST, PG_ROOT_DB, PG_ROOT_USER, PG_ROOT_PWD, 1).await?;
    pexec(&root_db, format!("{SQL_DIR}/{SQL_RECREATE}").as_str()).await?;

    // Populating the data (using app's user).
    let db = new_db_pool(PG_HOST, PG_DB, PG_USER, PG_PASS, PG_MAX_CONNS).await?;
    let mut paths: Vec<PathBuf> = fs::read_dir(SQL_DIR)?
        .into_iter()
        .filter_map(|f| f.ok().map(|e| e.path()))
        .collect();
    paths.sort();
    for path in paths {
        if let Some(path) = path.to_str() {
            if path.ends_with(".sql") && path.contains(SQL_RECREATE) {
                pexec(&db, &path).await?;
            }
        }
    }

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
    let conn = PgConnectOptions::new()
        .host(host)
        .port(5439)
        .username(user)
        .password(pwd)
        .ssl_mode(sqlx::postgres::PgSslMode::Disable)
        .connect()
        .await;
    dbg!("[new_db_pool] conn:", conn);

    PgPoolOptions::new()
        .max_connections(max_conns)
        .connect_timeout(Duration::from_millis(500))
        .connect(&conn_str)
        .await
}

async fn pexec(db: &DbPool, file: &str) -> Result<(), sqlx::Error> {
    // Read the file
    let content = fs::read_to_string(file).map_err(|e| {
        println!("ERROR reading {} file: {:?}", file, e);
        e
    })?;
    // FYI: This basic `;` is not bullet proof!
    let sqls: Vec<&str> = content.split(";").collect();
    for sql in sqls {
        match sqlx::query(&sql).execute(db).await {
            Ok(_) => (),
            Err(e) => {
                println!("ERROR pexec'ing sql stmt {}: {:?}", sql, e);
                return Err(e);
            }
        }
    }
    Ok(())
}

#[cfg(test)]
#[path = "../zz_tests/model_db.rs"]
mod tests;
