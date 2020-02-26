use std::io;

use actix_web::{web, App, HttpServer, Responder};

use dotenv::dotenv;

use crate::models::Status;

mod config;
mod models;

async fn status() -> impl Responder {
    web::HttpResponse::Ok().json(Status {
        status: "OK".to_string(),
    })
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv().ok();

    let config = config::Config::from_env().unwrap();

    println!(
        ">>> Starting server listening at http://{}:{}",
        config.server.host, config.server.port
    );

    HttpServer::new(|| App::new().route("/", web::get().to(status)))
        .bind(format!("{}:{}", config.server.host, config.server.port))?
        .run()
        .await
}
