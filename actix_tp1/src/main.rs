use actix_web::{App, HttpRequest, HttpServer, middleware, web};

fn index(req: HttpRequest) -> &'static str {
    println!("req: {:?}", req);
    "Index"
}

fn main() -> std::io::Result<()> {
    
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(||{
        App::new()
        .wrap(middleware::Logger::default())
        .service(web::resource("/index.html").to(|| "Index page"))
        .service(web::resource("/").to(index))
    })
    .bind("127.0.0.1:8080")?
    .run()

}
