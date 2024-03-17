use leptos::*;
use leptos_meta::*;
use leptos_router::{Route, Router, Routes};

use crate::components::{blog_previews::BlogPreviews, edit_post::EditPost, view_post::ViewPost};

#[component]
pub fn Navbar() -> impl IntoView {
    view! {
        <nav>"The navbar"</nav>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos-xp.css"/>

        // sets the document title
        <Title text="Leptos Xp"/>

        <Navbar/>

        <Router>
            <main>
                <Routes>
                    <Route path="" view=BlogPreviews />
                    <Route path="/edit/:post_id?" view=EditPost />
                    <Route path="/view/:post_id?" view=ViewPost />
                </Routes>
            </main>
        </Router>
    }
}
