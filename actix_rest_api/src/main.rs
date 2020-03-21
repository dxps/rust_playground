#[macro_use]
extern crate log;

use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use listenfd::ListenFd;

mod user;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // collect the vars from `.env` file for later usage
    dotenv().ok();
    // initiate the env logger to see at stdout and stderr
    // the messages sent by `log` crate's macros (like `info!()`)
    env_logger::init();

    let mut listenfd = ListenFd::from_env();

    let mut server = HttpServer::new(|| {
        // init the App
        App::new()
            // init the routes
            .configure(user::init_routes)
    });

    let host = std::env::var("HOST").expect("Host not defined");
    let port = std::env::var("PORT").expect("Port not defined");

    server = match listenfd.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => server.bind(format!("{}:{}", host, port))?,
    };

    info!(">>> Starting to listen on {}:{}", host, port);
    server.run().await
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Index")
}
