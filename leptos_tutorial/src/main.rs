use leptos::*;

fn main() {
    // Mount the App compnent to the <body>.
    mount_to_body(|| {
        view! {
            <App/>
        }
    })
}

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    let click_handler = move |_| {
        set_count.update(|n| *n += 1);
    };
    view! {
        <button
            on:click=click_handler>
            // Text nodes in RSX are wrapped in quotes, like a normal Rust string.
            // Rust expressions are wrapped in curly braces.
            // Since signals are functions, {count} is a reactive shorthand
            // to the closure {move || count.get()}
            "Click me: " {count}
        </button>
    }
}
