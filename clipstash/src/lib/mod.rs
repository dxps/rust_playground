pub mod data;
pub mod domain;
pub mod service;
pub mod web;

// Reexporting some types to have them accessible from here
// instead of having to go deeper into the domain model.
pub use data::DataError;
pub use domain::clip::field::ShortCode;
pub use domain::clip::{Clip, ClipError};
pub use domain::time::Time;
use rocket::fs::FileServer;
use rocket::{Build, Rocket};
pub use service::ServiceError;

use data::AppDatabase;
use web::renderer::Renderer;

pub struct RocketConfig {
    pub renderer: Renderer<'static>,
    pub database: AppDatabase,
}

pub fn rocket(config: RocketConfig) -> Rocket<Build> {
    rocket::build()
        .manage::<AppDatabase>(config.database)
        .manage::<Renderer>(config.renderer)
        .mount("/", web::http::routes())
        .mount("/static", FileServer::from("static"))
        .register("/", web::http::catcher::catchers())
}
