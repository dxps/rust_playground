use dioxus::prelude::*;

use crate::{
    model::{Comment, StoryItem, StoryPageData},
    state::PreviewState,
};

pub fn Home() -> Element {
    rsx! {
        div { display: "flex", flex_direction: "row", width: "100%",
            div { class:"bg-gray-100 h-screen", width: "50%", Stories {} }
            div { width: "50%", Preview {} }
        }
    }
}

pub fn Stories() -> Element {
    rsx! {
        StoryListing {
            story: StoryItem {
                id: 0,
                title: "hello hackernews".to_string(),
                url: None,
                text: None,
                by: "Author".to_string(),
                score: 0,
                descendants: 0,
                time: chrono::Utc::now(),
                kids: vec![],
                r#type: "".to_string(),
            }
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
        PreviewState::Loading => rsx! { "Loading ..." },
        PreviewState::Loaded(story) => {
            rsx! {
                div { padding: "0.5rem",
                    div { font_size: "1.5rem", a { href: story.item.url, "{story.item.title}" } }
                    div { dangerous_inner_html: story.item.text }
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
            div { dangerous_inner_html: "{comment.text}" }
            for kid in &comment.sub_comments {
                Comment { comment: kid.clone() }
            }
        }
    }
}

#[component]
fn StoryListing(story: ReadOnlySignal<StoryItem>) -> Element {
    let mut preview_state = consume_context::<Signal<PreviewState>>();
    let StoryItem {
        title,
        url,
        by,
        score,
        time,
        kids,
        ..
    } = &*story.read();

    let url = url.as_deref().unwrap_or_default();
    let hostname = url
        .trim_start_matches("https://")
        .trim_start_matches("http://")
        .trim_start_matches("www.");
    let score = format!("{score} {}", if *score == 1 { " point" } else { " points" });
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
            class: "hover:bg-white hover:rounded-md",
            padding: "0.5rem", position: "relative",
            onmouseenter: move |_| {
                *preview_state.write() = PreviewState::Loaded(StoryPageData {
                    item:story(),
                    comments: vec![],
                })
            },
            div { font_size: "1.5rem",
                a {
                    href: url,
                    onfocus: move |_event| {
                        *preview_state.write() = PreviewState::Loaded(StoryPageData {
                            item: story(),
                            comments: vec![]
                        })
                    },
                    "{title}" }
                a {
                    color: "gray",
                    href: "https://news.ycombinator.com/from?site={hostname}",
                    text_decoration: "none",
                    " ({hostname})"
                }
            }
            div { display: "flex", flex_direction: "row", color: "gray",
                div { "{score}" }
                div { padding_left: "0.5rem", "by {by}" }
                div { padding_left: "0.5rem", "{time}" }
                div { padding_left: "0.5rem", "{comments}" }
            }
        }
    }
}
