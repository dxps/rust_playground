use std::io;

use actix_web::{web, App, HttpServer};
use tokio_postgres::NoTls;

use dotenv::dotenv;

use crate::db::check_init_db_conn;
use crate::handlers::*;
use std::io::{Error, ErrorKind};

mod config;
mod db;
mod handlers;
mod models;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv().ok();

    let config = config::Config::from_env().unwrap();
    let pool = config.pg.create_pool(NoTls).unwrap();

    if !check_init_db_conn(pool.clone()).await {
        return Result::Err(Error::new(ErrorKind::Other, ""));
    } else {
        println!(">>> DB Connection is successful. ");
    }

    println!(
        ">>> DB Pool init state: available/size = {}/{}",
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
            .route("/todos{_:/?}", web::post().to(create_todo_list))
            .route("/todos/{id}/items{_:/?}", web::get().to(get_todo_items))
    })
    .bind(format!("{}:{}", config.server.host, config.server.port))?
    .run()
    .await
}
