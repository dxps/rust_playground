use super::http::{Request, Response, StatusCode};
use super::server::Handler;

pub struct MainHandler;

impl Handler for MainHandler {
    fn handle_request(&mut self, _: &Request) -> Response {
        Response::new(StatusCode::Ok, Some("<h1>It works!</h1>".to_string()))
    }
}
