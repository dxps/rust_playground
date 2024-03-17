use leptos::*;
use leptos_meta::*;
use leptos_router::{Route, Router, Routes};

use crate::components::{blog_previews::BlogPreviews, edit_post::EditPost, view_post::ViewPost};

#[component]
pub fn Navbar() -> impl IntoView {
    view! {
        <div class="bg-white text-gray-800 px-4 py-3">
            <div class="container mx-auto flex justify-between items-center">
                <a href="/">Home</a>
                <nav>
                    <ul class="flex space-x-4">
                        <li><a href="/">Blog</a></li>
                        <li><a href="/edit" class="hover:text-blue-700">Create</a></li>
                    </ul>
                </nav>
            </div>
        </div>
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
            <main class="bg-gray-50 text-gray-900 p-8 h-full">
                <Routes>
                    <Route path="" view=BlogPreviews />
                    <Route path="/edit/:post_id?" view=EditPost />
                    <Route path="/view/:post_id?" view=ViewPost />
                </Routes>
            </main>
        </Router>
    }
}
