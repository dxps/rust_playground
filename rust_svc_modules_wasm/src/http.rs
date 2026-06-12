use crate::plugin_manager::{
    DispatchRequest, PluginError, PluginManager, RegisterPlugin, UpdatePluginConfig,
};
use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
};
use serde::Serialize;

pub fn router(plugin_manager: PluginManager) -> Router {
    Router::new()
        .route("/healthz", get(healthz))
        .route("/plugins", get(list_plugins).post(register_plugin))
        .route(
            "/plugins/{id}",
            get(get_plugin)
                .delete(remove_plugin)
                .put(update_plugin_config),
        )
        .route("/plugins/{id}/load", post(load_plugin))
        .route("/plugins/{id}/unload", post(unload_plugin))
        .route("/plugins/{id}/dispatch", post(dispatch_plugin))
        .with_state(plugin_manager)
}

async fn healthz() -> &'static str {
    "ok"
}

async fn list_plugins(State(manager): State<PluginManager>) -> impl IntoResponse {
    Json(manager.list().await)
}

async fn get_plugin(
    State(manager): State<PluginManager>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, ApiError> {
    Ok(Json(manager.get(&id).await?))
}

async fn register_plugin(
    State(manager): State<PluginManager>,
    Json(input): Json<RegisterPlugin>,
) -> Result<impl IntoResponse, ApiError> {
    Ok((StatusCode::CREATED, Json(manager.register(input).await?)))
}

async fn remove_plugin(
    State(manager): State<PluginManager>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, ApiError> {
    Ok(Json(manager.remove(&id).await?))
}

async fn update_plugin_config(
    State(manager): State<PluginManager>,
    Path(id): Path<String>,
    Json(input): Json<UpdatePluginConfig>,
) -> Result<impl IntoResponse, ApiError> {
    Ok(Json(manager.update_config(&id, input).await?))
}

async fn load_plugin(
    State(manager): State<PluginManager>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, ApiError> {
    Ok(Json(manager.load(&id).await?))
}

async fn unload_plugin(
    State(manager): State<PluginManager>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, ApiError> {
    Ok(Json(manager.unload(&id).await?))
}

async fn dispatch_plugin(
    State(manager): State<PluginManager>,
    Path(id): Path<String>,
    Json(input): Json<DispatchRequest>,
) -> Result<impl IntoResponse, ApiError> {
    Ok(Json(manager.dispatch(&id, input).await?))
}

struct ApiError(PluginError);

#[derive(Serialize)]
struct ErrorBody {
    error: String,
}

impl From<PluginError> for ApiError {
    fn from(value: PluginError) -> Self {
        Self(value)
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let status = match self.0 {
            PluginError::AlreadyExists(_) => StatusCode::CONFLICT,
            PluginError::NotFound(_) => StatusCode::NOT_FOUND,
            PluginError::AlreadyLoaded(_) | PluginError::NotLoaded(_) => StatusCode::CONFLICT,
            PluginError::EmptyId
            | PluginError::MissingComponent(_)
            | PluginError::ConfigRejected { .. }
            | PluginError::RequestRejected { .. } => StatusCode::BAD_REQUEST,
            PluginError::Runtime { .. } => StatusCode::INTERNAL_SERVER_ERROR,
        };

        (
            status,
            Json(ErrorBody {
                error: self.0.to_string(),
            }),
        )
            .into_response()
    }
}
