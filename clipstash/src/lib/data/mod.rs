// Including these modules in the module tree.
pub mod clip;
pub mod database;
pub mod dbid;
pub mod query;

// And reexporting for shorter use path.
pub use clip::Clip;
pub use database::{DataError, Database};
pub use dbid::DbId;

use sqlx::Sqlite;

// Type aliases to hide sqlite.
pub type AppDatabase = Database<Sqlite>;
pub type DatabasePool = sqlx::sqlite::SqlitePool;
pub type Transaction<'t> = sqlx::Transaction<'t, Sqlite>;
pub type AppDatabaseRow = sqlx::sqlite::SqliteRow;
pub type AppqueryResult = sqlx::sqlite::SqliteQueryResult;
