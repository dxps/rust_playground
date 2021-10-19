pub mod api;
pub mod ctx;
pub mod forms;
pub mod hit_count;
pub mod http;
pub mod page_error;
pub mod renderer;

// Reexporting
pub use hit_count::HitCounter;

pub const PASSWORD_COOKIE: &str = "password";

#[cfg(test)]
pub mod test {
    use crate::{data, domain::maintenance::Maintenance, test::async_runtime, RocketConfig};
    use rocket::local::blocking::Client;

    pub fn config() -> RocketConfig {
        use crate::web::{hit_count::HitCounter, renderer::Renderer};

        let rt = async_runtime();
        let renderer = Renderer::new("templates/".into());
        let database = data::database::test::new_db(rt.handle());
        let maintenance = Maintenance::spawn(database.get_pool().clone(), rt.handle().clone());
        let hit_counter = HitCounter::new(database.get_pool().clone(), rt.handle().clone());
        RocketConfig {
            renderer,
            database,
            hit_counter,
            maintenance,
        }
    }

    pub fn client() -> Client {
        let config = config();
        Client::tracked(crate::rocket(config)).expect("failed to build rocket instance")
    }
}
