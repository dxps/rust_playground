use warp::Filter;

#[tokio::main]
async fn main() {
    let hello = warp::get().map(|| format!("Hello, world!"));

    warp::serve(hello).run(([127, 0, 0, 1], 8083)).await;
}
