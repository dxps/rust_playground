// Including in the module tree.
pub mod clip;
pub mod database;
pub mod dbid;
pub mod get_clip;
pub mod new_clip;
pub mod update_clip;

// Reexporting.
pub use clip::Clip;
pub use database::{DataError, Database};
pub use dbid::DbId;
pub use get_clip::{get_clip, GetClip};
pub use new_clip::{new_clip, NewClip};
pub use update_clip::{update_clip, UpdateClip};

use sqlx::Sqlite;

// Type aliases to hide sqlite.
pub type AppDatabase = Database<Sqlite>;
pub type DatabasePool = sqlx::sqlite::SqlitePool;
pub type Transaction<'t> = sqlx::Transaction<'t, Sqlite>;
pub type AppDatabaseRow = sqlx::sqlite::SqliteRow;
pub type AppqueryResult = sqlx::sqlite::SqliteQueryResult;

/// A type alias for a result that might contain `DataError`.
type Result<T> = std::result::Result<T, DataError>;
