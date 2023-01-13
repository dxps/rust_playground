use axum::{
    routing::{delete, get},
    Router,
};
use shuttle_service::{error::CustomError, tracing, ShuttleAxum};
use sqlx::{Executor, PgPool};
use sync_wrapper::SyncWrapper;

use crate::txt_handlers::{create_test, delete_test, list_tests};

async fn hello_world() -> &'static str {
    "Hello, world!"
}

#[shuttle_service::main]
async fn axum(#[shuttle_shared_db::Postgres] pool: PgPool) -> ShuttleAxum {
    //
    pool.execute(include_str!("../db/schema.sql"))
        .await
        .map_err(|e| CustomError::new(e).context("failed to execute schema.sql"))?;

    let router = router(pool).await;
    let sync_wrapper = SyncWrapper::new(router);
    tracing::info!("Starting Axum server ...");

    Ok(sync_wrapper)
}

async fn router(pool: PgPool) -> Router {
    Router::new()
        .route("/hello", get(hello_world))
        .route("/txt", get(list_tests).post(create_test))
        .route("/txt/:id", delete(delete_test))
        .with_state(pool)
}
