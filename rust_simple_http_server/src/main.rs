// TODO: temporary used
#![allow(dead_code)]

use main_handler::MainHandler;
use server::Server;

mod http;
mod main_handler;
mod server;

fn main() {
    let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
    let public_path = std::env::var("PUBLIC_PATH").unwrap_or(default_path);
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run(MainHandler::new(public_path));
}
