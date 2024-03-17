use leptos::*;

use crate::model::Post;

#[component]
pub fn BlogPost(post: Post) -> impl IntoView {
    view! {
        <div class="block p-10">
            <div class="text-2xl pb-4">{&post.title}</div>
            <div>{&post.content}</div>
        </div>
    }
}
