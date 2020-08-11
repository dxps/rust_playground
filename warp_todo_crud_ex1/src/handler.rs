use crate::{db, DbPool};
use warp::{http::StatusCode, reject, Rejection, Reply};

pub async fn health_handler(db_pool: DbPool) -> std::result::Result<impl Reply, Rejection> {
    let db = db::get_db_conn(&db_pool)
        .await
        .map_err(|e| reject::custom(e))?;

    db.execute("SELECT 1", &[])
        .await
        .map_err(|e| reject::custom(DBQueryError(e)))?;
    Ok(StatusCode::OK)
}
