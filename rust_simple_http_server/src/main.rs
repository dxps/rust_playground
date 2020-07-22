// TODO: temporary used
#![allow(dead_code)]

use main_handler::MainHandler;
use server::Server;

mod http;
mod main_handler;
mod server;

fn main() {
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run(MainHandler);
}
