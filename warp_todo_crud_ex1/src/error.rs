use mobc_postgres::tokio_postgres;
use serde_derive::Serialize;
use std::convert::Infallible;
use thiserror::Error;
use warp::{http::StatusCode, Rejection, Reply};

/// Application custom error catalog.
#[derive(Error, Debug)]
pub enum Error {
    #[error("error getting connection from DB pool: {0}")]
    DbPoolError(mobc::Error<tokio_postgres::Error>),
    #[error("error executing DB query: {0}")]
    DbQueryError(#[from] tokio_postgres::Error),
    #[error("error creating table: {0}")]
    DbInitError(tokio_postgres::Error),
    #[error("error reading file: {0}")]
    ReadFileError(#[from] std::io::Error),
}

/// The friendly error response that is sent outside.
#[derive(Serialize)]
struct ErrorResponse {
    message: String,
}

// Any application custom error should be seen as a WARP rejection, thus it implements `Reject` trait.
impl warp::reject::Reject for Error {}

/// The rejection handler.
pub async fn handle_rejection(err: Rejection) -> std::result::Result<impl Reply, Infallible> {
    let code;
    let message;

    if err.is_not_found() {
        code = StatusCode::NOT_FOUND;
        message = "Not Found";
    } else if err
        .find::<warp::filters::body::BodyDeserializeError>()
        .is_some()
    {
        code = StatusCode::BAD_REQUEST;
        message = "Invalid Body";
    } else if let Some(e) = err.find::<Error>() {
        match e {
            Error::DbQueryError(_) => {
                code = StatusCode::BAD_REQUEST;
                message = "Could not execute request";
            }
            _ => {
                eprintln!("Unhandled app error: {:?}", err);
                code = StatusCode::INTERNAL_SERVER_ERROR;
                message = "Internal Server Error";
            }
        }
    } else if err.find::<warp::reject::MethodNotAllowed>().is_some() {
        code = StatusCode::METHOD_NOT_ALLOWED;
        message = "Method Not Allowed";
    } else {
        eprintln!("Unhandled error: {:?}", err);
        code = StatusCode::INTERNAL_SERVER_ERROR;
        message = "Internal Server Error";
    }

    let json = warp::reply::json(&ErrorResponse {
        message: message.into(),
    });

    Ok(warp::reply::with_status(json, code))
}
