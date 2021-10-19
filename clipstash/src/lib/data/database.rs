use sqlx::Sqlite;

use super::DatabasePool;

/// A wrappter around a database connection pool.
pub struct Database<D: sqlx::Database>(sqlx::Pool<D>);

// Implementation for Sqlite.
impl Database<Sqlite> {
    /// Create a new `Database` based on the connection string.
    pub async fn new(connection_str: &str) -> Self {
        let pool = sqlx::sqlite::SqlitePoolOptions::new()
            .connect(connection_str)
            .await;
        match pool {
            Ok(pool) => Self { 0: pool },
            Err(err) => {
                eprintln!("Database new error: {}", err);
                eprintln!(
                    "If the database has not yet been created, run:\n  sqlx database setup\n"
                );
                panic!("Database connection error.");
            }
        }
    }

    /// Get a reference to the database connection pool.
    pub fn get_pool(&self) -> &DatabasePool {
        &self.0
    }
}

#[derive(Debug, thiserror::Error)]
pub enum DataError {
    #[error("database error: {0}")]
    Database(#[from] sqlx::Error),
}

#[cfg(test)]
pub mod test {
    use tokio::runtime::Handle;

    use crate::data::{AppDatabase, Database};

    /// A testing helper for creating a database.
    pub fn new_db(handle: &Handle) -> AppDatabase {
        use sqlx::migrate::Migrator;
        use std::path::Path;

        handle.block_on(async move {
            let db = Database::new(":memory:").await;
            let migrator = Migrator::new(Path::new("./migrations")).await.unwrap();
            let pool = db.get_pool();
            migrator.run(pool).await.unwrap();
            db
        })
    }
}
