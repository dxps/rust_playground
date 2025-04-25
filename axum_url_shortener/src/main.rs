use axum::{
    Json, Router,
    extract::Path,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

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

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(handler))
        .route("/length/{url}", get(url_length))
        .route("/validate-url", post(validate_url));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
