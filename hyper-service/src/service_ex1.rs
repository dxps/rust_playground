use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use std::convert::Infallible;
use std::net::SocketAddr;

async fn handle(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    println!(">>> received {:?}", req);
    Ok(Response::new("Rust hyper service".into()))
}

#[tokio::main]
async fn main() {
    // For every connection, a `Service` will handle incoming HTTP requests on that connection.
    let service = make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(handle)) });

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    let server = Server::bind(&addr).serve(service);

    println!(">>> Listening on http://{}", addr);

    if let Err(e) = server.await {
        eprintln!(">>> Server error: {}", e);
    }
}
