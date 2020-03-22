use diesel::{pg::PgConnection, r2d2::ConnectionManager};
use r2d2;

use lazy_static::lazy_static;

use crate::api_error::ApiError;

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type DbConn = r2d2::PooledConnection<ConnectionManager<PgConnection>>;

embed_migrations!();

lazy_static! {
    static ref POOL: Pool = {
        let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not set");
        let mgr = ConnectionManager::<PgConnection>::new(db_url);
        Pool::new(mgr).expect("Failed to create db conn pool")
    };
}

pub fn init() {
    info!("Initializing DB");
    lazy_static::initialize(&POOL);
    let conn = get_conn().expect("Error getting db conn");
    embedded_migrations::run(&conn).unwrap();
}

/// Get a db connection from the pool.
pub fn get_conn() -> Result<DbConn, ApiError> {
    POOL.get()
        .map_err(|err| ApiError::new(500, format!("Error getting db conn: {}", err)))
}
