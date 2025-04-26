mod urls;

use axum::{
    Json, Router,
    body::Body,
    extract::Path,
    http::StatusCode,
    response::Response,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use std::{net::SocketAddr, time::Duration};
use tower_http::{timeout::TimeoutLayer, trace::TraceLayer};
use tracing_subscriber;

async fn handler() -> &'static str {
    "Hello, Axum!"
}

#[derive(Serialize)]
struct UrlLengthJsonResponse {
    length: u32,
}

async fn url_length(Path(url): Path<String>) -> Json<UrlLengthJsonResponse> {
    Json(UrlLengthJsonResponse {
        length: url.len() as u32,
    })
}

#[derive(Deserialize)]
struct UrlRequest {
    url: String,
}

#[derive(Serialize)]
struct UrlResponse {
    url: String,
}

async fn validate_url(Json(payload): Json<UrlRequest>) -> Json<UrlResponse> {
    let is_valid = payload.url.starts_with("http://") || payload.url.starts_with("https://");
    if !is_valid {
        Json(UrlResponse {
            url: "Invalid URL".to_string(),
        })
    } else {
        Json(UrlResponse { url: payload.url })
    }
}

#[derive(Serialize)]
struct ShortenUrlResponse {
    shortened_url: String,
}

#[derive(Serialize)]
struct LongUrlResponse {
    long_url: String,
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

async fn shorten_url(payload: String) -> Json<ShortenUrlResponse> {
    // Just an example of how to generate a timeout (that responds
    // back to the client with HTTP 408 Request timeout).
    // tokio::time::sleep(Duration::from_secs(4)).await;

    let short_id = urls::encode_short_url(&payload);
    tracing::info!("short_id: {}", short_id);
    Json(ShortenUrlResponse {
        shortened_url: short_id,
    })
}

async fn unshorten_url(Path(short_id): Path<String>) -> Response {
    let long_url = urls::decode_long_url(&short_id);
    match long_url {
        Ok(url) => {
            tracing::info!(url, "long_url");
            Response::builder()
                .status(StatusCode::FOUND)
                .header("Content-Type", "application/json")
                .body(Body::from(
                    serde_json::to_string(&LongUrlResponse { long_url: url }).unwrap(),
                ))
                .unwrap()
        }
        Err(e) => {
            tracing::error!("error: {}", e);
            Response::builder()
                .status(StatusCode::NOT_FOUND)
                .header("Content-Type", "application/json")
                .body(Body::from(
                    serde_json::to_string(&ErrorResponse {
                        error: e.to_string(),
                    })
                    .unwrap(),
                ))
                .unwrap()
        }
    }
}

#[tokio::main]
async fn main() {
    // Initialize the logger.
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(handler))
        .route("/length/{url}", get(url_length))
        .route("/validate-url", post(validate_url))
        .route("/shorten", post(shorten_url))
        .route("/unshorten/{short_id}", get(unshorten_url))
        .layer(TraceLayer::new_for_http())
        .layer(TimeoutLayer::new(Duration::from_secs(3)));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    tracing::info!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}
