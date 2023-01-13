use std::error::Error;

use axum::http::StatusCode;

pub type AppError = (StatusCode, String);

pub fn err<E>(status_code: StatusCode) -> impl FnOnce(E) -> AppError
where
    E: Error,
{
    move |error: E| (status_code, error.to_string())
}

pub fn err_internal<E>() -> impl FnOnce(E) -> AppError
where
    E: Error,
{
    err(StatusCode::INTERNAL_SERVER_ERROR)
}
