use dioxus::prelude::*;

use crate::{
    fetch_data::{get_stories, resolve_story},
    model::{Comment, StoryItem},
    state::PreviewState,
};

pub fn Home() -> Element {
    rsx! {
        div { display: "flex", flex_direction: "row", width: "100%",
            div { class:"bg-gray-100 h-screen overflow-y-auto", width: "50%", Stories {} }
            div { class:"h-screen overflow-y-auto", width: "50%", Preview {} }
        }
    }
}

pub fn Stories() -> Element {
    // Fetch the top 10 stories.
    let stories = use_resource(move || get_stories(10));

    // Check if the future is resolved.
    match &*stories.read_unchecked() {
        Some(Ok(list)) => {
            rsx! {
                div {
                    for story in list {
                        StoryListing { story: story.clone() }
                    }
                }
            }
        }
        Some(Err(err)) => {
            rsx! { "Failed to fetch the stories due to {err}" }
        }
        None => {
            rsx! { "Loading the stories ..." }
        }
    }
}

fn Preview() -> Element {
    let preview_state = consume_context::<Signal<PreviewState>>();

    match preview_state() {
        PreviewState::Unset => {
            rsx! { div { color: "gray", font_size: "0.9rem", padding: "0.5rem",
               "Hover over a story to preview it here" }
            }
        }
        PreviewState::Loading => rsx! {
            div { font_size: "0.9rem", padding: "0.5rem", "Loading the story ..." }
        },
        PreviewState::Loaded(story) => {
            rsx! {
                div { padding: "0.5rem",
                    div { font_size: "1.4rem", padding_left: "0.5rem",
                        a { href: story.item.url, "{story.item.title}" }
                    }
                    div { font_size: "0.9rem", dangerous_inner_html: story.item.text }
                    for comment in &story.comments {
                        Comment { comment: comment.clone() }
                    }
                }
            }
        }
    }
}

#[component]
fn Comment(comment: Comment) -> Element {
    rsx! {
        div { padding: "0.5rem",
            div { color: "gray", "by {comment.by}" }
            div { font_size: "0.9rem", dangerous_inner_html: "{comment.text}" }
            for kid in &comment.sub_comments {
                Comment { comment: kid.clone() }
            }
        }
    }
}

#[component]
fn StoryListing(story: ReadOnlySignal<StoryItem>) -> Element {
    let preview_state = consume_context::<Signal<PreviewState>>();
    let StoryItem {
        title,
        url,
        by,
        score,
        time,
        kids,
        id,
        ..
    } = story();

    let full_story = use_signal(|| None);

    let url = url.as_deref().unwrap_or_default();
    let hostname = url
        .trim_start_matches("https://")
        .trim_start_matches("http://")
        .trim_start_matches("www.");
    let score = format!("{score} {}", if score == 1 { " point" } else { " points" });
    let comments = format!(
        "{} {}",
        kids.len(),
        if kids.len() == 1 {
            " comment"
        } else {
            " comments"
        }
    );
    let time = time.format("%D %l:%M %p");

    rsx! {
        div {
            class: "hover:bg-white hover:rounded-lg",
            margin: "0.6rem", padding: "0.5rem", position: "relative",
            onmouseenter: move |_| { resolve_story(full_story, preview_state, id) },
            div { font_size: "0.7rem",
                a { font_size: "1rem",
                    href: url,
                    "{title}" }
                a {
                    color: "gray",
                    href: "https://news.ycombinator.com/from?site={hostname}",
                    text_decoration: "none",
                    " ({hostname})"
                }
            }
            div { display: "flex", flex_direction: "row", color: "gray", font_size: "0.7rem",
                div { "{score}" }
                div { padding_left: "0.5rem", "by {by}" }
                div { padding_left: "0.5rem", "{time}" }
                div { padding_left: "0.5rem", "{comments}" }
            }
        }
    }
}
