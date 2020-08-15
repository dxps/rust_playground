use crate::{
    data::{TodoRequest, TodoResponse, TodoUpdateRequest},
    db,
    error::Error::DbQueryError,
    DbPool, Result,
};
use serde::Deserialize;
use warp::reply::json;
use warp::{http::StatusCode, reject, Rejection, Reply};

#[derive(Deserialize)]
pub struct SearchQuery {
    search: Option<String>,
}

pub async fn health_handler(db_pool: DbPool) -> std::result::Result<impl Reply, Rejection> {
    let db = db::get_db_conn(&db_pool).await.map_err(reject::custom)?;

    db.execute("SELECT 1", &[])
        .await
        .map_err(|e| reject::custom(DbQueryError(e)))?;
    Ok(StatusCode::OK)
}

pub async fn create_todo_handler(body: TodoRequest, db_pool: DbPool) -> Result<impl Reply> {
    Ok(json(&TodoResponse::of(
        db::create_todo(&db_pool, body)
            .await
            .map_err(reject::custom)?,
    )))
}

pub async fn list_todos_handler(query: SearchQuery, db_pool: DbPool) -> Result<impl Reply> {
    let todos = db::fetch_todos(&db_pool, query.search)
        .await
        .map_err(reject::custom)?;
    Ok(json::<Vec<_>>(
        &todos.into_iter().map(TodoResponse::of).collect(),
    ))
}

pub async fn update_todo_handler(
    id: i32,
    body: TodoUpdateRequest,
    db_pool: DbPool,
) -> Result<impl Reply> {
    Ok(json(&TodoResponse::of(
        db::update_todo(&db_pool, id, body)
            .await
            .map_err(reject::custom)?,
    )))
}

pub async fn delete_todo_handler(id: i32, db_pool: DbPool) -> Result<impl Reply> {
    db::delete_todo(&db_pool, id)
        .await
        .map_err(reject::custom)?;
    Ok(StatusCode::OK)
}
