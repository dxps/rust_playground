#![allow(non_snake_case)]

mod components;
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
    // #[route("/blog/:id")]
    // Blog { id: i32 },
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

// #[component]
// fn Blog(id: i32) -> Element {
//     rsx! {
//         Link { to: Route::Home {}, "Go to counter" }
//         "Blog post {id}"
//     }
// }

// #[component]
// fn Home() -> Element {
//     let mut count = use_signal(|| 0);

//     rsx! {
//         Link {
//             to: Route::Blog {
//                 id: count()
//             },
//             "Go to blog"
//         }
//         div {
//             h1 { "High-Five counter: {count}" }
//             button { onclick: move |_| count += 1, "Up high!" }
//             button { onclick: move |_| count -= 1, "Down low!" }
//         }
//     }
// }
