// fn main() -> std::io::Result<()> {
fn main() {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    // let app = MessageApp::new(8080);
    // app.run()
}
