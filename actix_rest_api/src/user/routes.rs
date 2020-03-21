use crate::user::model::User;
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use serde_json::json;

#[get("/users")]
async fn get_all_users() -> impl Responder {
    HttpResponse::Ok().json(vec![
        User {
            id: 1,
            email: "john@mail.com".to_string(),
        },
        User {
            id: 2,
            email: "jane@mail.com".to_string(),
        },
    ])
}

#[post("/users")]
async fn create(user: web::Json<User>) -> impl Responder {
    HttpResponse::Created().json(user.into_inner())
}

#[put("/users")]
async fn update(user: web::Json<User>) -> impl Responder {
    HttpResponse::Created().json(user.into_inner())
}

#[delete("/users/{id}")]
async fn delete() -> impl Responder {
    HttpResponse::Ok().json(json!({ "message": "Deleted"}))
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_all_users);
    cfg.service(create);
    cfg.service(update);
    cfg.service(delete);
}
