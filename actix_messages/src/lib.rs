#[macro_use]
extern crate actix_web;

use actix_web::{middleware, App, HttpRequest, HttpResponse, HttpServer};
use serde::Serialize;
use std::cell::Cell;
use std::sync::atomic::AtomicUsize;
use std::sync::{Arc, Mutex};

// -------------------- Application State --------------------

static SERVER_COUNTER: AtomicUsize = AtomicUsize::new(0);

// Each worker gets its own instance of this struct.
struct AppState {
    server_id: usize,
    request_count: Cell<usize>,
    messages: Arc<Mutex<Vec<String>>>,
}

// -----------------------------------------------------------

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
                .wrap(middleware::DefaultHeaders::new().header("x-myheader-1", "koohah"))
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
