use std::io;

use actix_web::{web, App, HttpServer};
use tokio_postgres::NoTls;

use dotenv::dotenv;

use crate::handlers::*;

mod config;
mod db;
mod handlers;
mod models;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv().ok();

    let config = config::Config::from_env().unwrap();

    let pool = config.pg.create_pool(NoTls).unwrap();
    println!(
        ">>> DB Pool status: available/size = {}/{}",
        pool.status().available,
        pool.status().size
    );

    println!(
        ">>> Starting server listening at http://{}:{}",
        config.server.host, config.server.port
    );

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .route("/", web::get().to(status))
            .route("/todos{_:/?}", web::get().to(get_todos))
    })
    .bind(format!("{}:{}", config.server.host, config.server.port))?
    .run()
    .await
}
