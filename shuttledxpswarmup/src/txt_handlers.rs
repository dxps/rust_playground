use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::Deserialize;
use sqlx::PgPool;

use crate::{
    errors::{err_internal, AppError},
    model::Test,
};

#[derive(Deserialize)]
pub struct CreateTextRq {
    txt: String,
}

pub async fn create_test(
    State(db): State<PgPool>,
    Json(dto): Json<CreateTextRq>,
) -> Result<Json<Test>, AppError> {
    let test = sqlx::query_as("INSERT INTO test (txt) VALUES ($1) RETURNING id, txt")
        .bind(&dto.txt.as_str())
        .fetch_one(&db)
        .await
        .map_err(err_internal())?;

    Ok(Json(test))
}

pub async fn list_tests(State(db): State<PgPool>) -> Result<Json<Vec<Test>>, AppError> {
    let tests = sqlx::query_as("SELECT id, txt FROM test")
        .fetch_all(&db)
        .await
        .map_err(err_internal())?;
    Ok(Json(tests))
}

pub async fn delete_test(
    State(db): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<StatusCode, AppError> {
    sqlx::query("DELETE FROM test WHERE id = $1")
        .bind(id)
        .execute(&db)
        .await
        .map_err(err_internal())?;
    Ok(StatusCode::NO_CONTENT)
}
