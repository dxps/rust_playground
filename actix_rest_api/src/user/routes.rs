use crate::api_error::ApiError;
use crate::user::model::{User, UserDto};
use actix_web::{delete, get, post, put, web, HttpResponse};
use serde_json::json;
use uuid::Uuid;

#[get("/users")]
async fn get_all() -> Result<HttpResponse, ApiError> {
    let users = User::find_all()?;
    Ok(HttpResponse::Ok().json(users))
}

#[get("/users/{id}")]
async fn get(id: web::Path<Uuid>) -> Result<HttpResponse, ApiError> {
    let user = User::find(id.into_inner())?;
    Ok(HttpResponse::Ok().json(user))
}

#[post("/users")]
async fn create(user_dto: web::Json<UserDto>) -> Result<HttpResponse, ApiError> {
    let user = User::create(user_dto.into_inner())?;
    Ok(HttpResponse::Created().json(user))
}

#[put("/users/{id}")]
async fn update(id: web::Path<Uuid>, user_dto: web::Json<UserDto>) -> Result<HttpResponse, ApiError> {
    let user = User::update(id.into_inner(), user_dto.into_inner())?;
    Ok(HttpResponse::Ok().json(user))
}

#[delete("/users/{id}")]
async fn delete(id: web::Path<Uuid>) -> Result<HttpResponse, ApiError> {
    let count = User::delete(id.into_inner())?;
    Ok(HttpResponse::Ok().json(json!({ "deleted_count": count })))
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_all);
    cfg.service(get);
    cfg.service(create);
    cfg.service(update);
    cfg.service(delete);
}
