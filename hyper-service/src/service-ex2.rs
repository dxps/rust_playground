// This is another example of running a simple API with Hyper.
// Based on [Skip the Framework: Build a Simple Rust API with Hyper](https://dev.to/deciduously/skip-the-framework-build-a-simple-rust-api-with-hyper-4jf5)
// For now, it needs to be adapted to the updated versions of Future and Hyper.
//

use futures::{future, Future, Stream};
use hyper::{
    client::HttpConnector, rt, service::service_fn, Body, Client, Request, Response, Server,
};

type GenericError = Box<dyn std::error::Error + Send + Sync>;
type ResponseFuture = Box<dyn Future<Item = Response<Body>, Error = GenericError> + Send + Unpin>;

fn router(req: Request<Body>, _client: &Client<HttpConnector>) -> ResponseFuture {
    // TODO: to be implemented
    unimplemented!()
}

fn main() {
    pretty_env_logger::init();
    let addr = "127.0.0.1:3000".parse().unwrap(); // using 'turbofish' (to explicit the type) is not needed here

    rt::run(future::lazy(move |_| {
        // create a Client for the service
        let client = Client::new();

        // define a service
        let service = move || {
            // a client clone is moved in this closure
            let client = client.clone();
            service_fn(move |req| router(req, &client))
        };

        // define the server
        let server = Server::bind(&addr)
            .serve(service)
            .map_err(|e| eprintln!(">>> Server error: {}", e));

        println!(">>> Listening on http://{}", addr);
        server
    }));
}
