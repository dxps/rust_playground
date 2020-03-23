#[macro_use]
extern crate actix_web;

use actix_web::{middleware, App, HttpRequest, HttpResponse, HttpServer};
use serde::Serialize;

pub struct MessageApp {
    port: u16,
}

impl MessageApp {
    pub fn new(port: u16) -> Self {
        MessageApp { port }
    }

    pub async fn run(&self) -> std::io::Result<()> {
        println!("Starting HTTP Server on 127.0.0.1:{}", self.port);
        HttpServer::new(move || {
            App::new()
                .wrap(middleware::Logger::default())
                .service(index)
        })
        .bind(("127.0.0.1", self.port))?
        .workers(4)
        .run()
        .await
    }
}

#[derive(Serialize)]
struct IndexResponse {
    message: String,
}

#[get("/")]
async fn index(req: HttpRequest) -> std::io::Result<HttpResponse> {
    let hello = req
        .headers()
        .get("hello")
        .and_then(|val| val.to_str().ok())
        .unwrap_or_else(|| "world");

    Ok(HttpResponse::Ok().json(IndexResponse {
        message: hello.to_owned(),
    }))
}
