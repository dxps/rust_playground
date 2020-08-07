use warp::hyper::StatusCode;
use warp::Filter;

mod data;
mod db;
mod error;
mod handler;

#[tokio::main]
async fn main() {
    println!("Starting up ...");

    let health_route = warp::path!("health").map(|| StatusCode::OK);

    let routes = health_route.with(warp::cors().allow_any_origin());

    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
}
