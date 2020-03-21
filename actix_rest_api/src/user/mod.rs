// declaring the modules
mod model;
mod routes;

// exposing to the outside world
pub use model::User;
pub use routes::init_routes;
