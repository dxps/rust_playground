#![allow(non_snake_case)]

mod components;
mod fetch_data;
mod model;
mod state;

use components::Home;

use dioxus::prelude::*;
use log::LevelFilter;

use crate::state::PreviewState;

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
}

fn main() {
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");
    console_error_panic_hook::set_once();

    launch(App);
}

fn App() -> Element {
    use_context_provider(|| Signal::new(PreviewState::Unset));
    rsx! {
        Router::<Route> {}
    }
}
