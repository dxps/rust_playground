use rocket::{get, routes};

#[get("/")]
async fn index() -> &'static str {
    "Hello, Rocket"
}

#[rocket::main]
async fn main() {
    match rocket::build().mount("/", routes![index]).launch().await {
        Ok(_) => (),
        Err(e) => println!("Startup error: {}", e),
    }
}
