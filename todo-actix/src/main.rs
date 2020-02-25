mod models;

use actix_web::{web, App, HttpServer, Responder};
use std::io;
use crate::models::Status;

async fn status() -> impl Responder {
    web::HttpResponse::Ok()
        .json(Status{ status: "OK".to_string() })
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    HttpServer::new(|| App::new().route("/", web::get().to(status)))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
