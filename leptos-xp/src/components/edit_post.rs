use leptos::*;
use leptos_router::{use_params, Params};
use serde::{Deserialize, Serialize};

use crate::{components::blog_post::BlogPost, model::Post};

#[derive(Params, PartialEq, Eq, Debug, Clone, Serialize, Deserialize)]
struct EditPostParams {
    post_id: Option<String>,
}

#[component]
pub fn EditPost() -> impl IntoView {
    let params = use_params::<EditPostParams>();

    let (post, set_post) = create_signal(Post::new_empty());

    view! {
        <div class="flex h-screen">
            <div class="min-w-[50%] max-h-[90%] text-gray-800 bg-gray-200 p-10 rounded-md">
                <form>
                    <label class="block mb-4">
                        <span>Title</span>
                        <input class="mt-1 p-2 w-full" type="text" id="title" name="title"
                            on:input=move |ev| {
                                set_post.update(|post| post.title = event_target_value(&ev))
                            }
                            prop:value={ move || post.get().title }
                        />
                    </label>
                    <label class="block mb-4">
                        <span>Content</span>
                        <textarea class="mt-1 p-2 w-full" id="text" name="text"
                            on:input=move |ev| {
                                set_post.update(|post| post.content = event_target_value(&ev))
                            }
                            prop:value={ move || post.get().content}
                        />
                    </label>
                </form>
            </div>
            <div>
                { move || view! { <BlogPost post=post.get() /> } }
            </div>
        </div>
    }
}
