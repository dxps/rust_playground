use ::actix_messages::MessageApp;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let app = MessageApp::new(8080);
    app.run().await
}
