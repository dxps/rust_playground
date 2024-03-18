use chrono::Local;
use leptos::{server, ServerFnError};

use crate::model::Post;

#[cfg(feature = "ssr")]
pub mod ssr {
    use leptos::ServerFnError;
    use sqlx::{
        sqlite::{SqlitePool, SqlitePoolOptions},
        Connection, SqliteConnection,
    };

    pub async fn db_pool() -> Result<SqlitePool, ServerFnError> {
        Ok(SqlitePoolOptions::new()
            .connect("sqlite:post.db")
            .await
            .expect("Could not make db pool."))
    }
}

#[server(UpsertPost, "/api")]
pub async fn upsert_post(
    id: Option<String>,
    dt: String,
    image_url: String,
    title: String,
    content: String,
) -> Result<String, ServerFnError> {
    // TODO: SQL to insert or update.
    let db_pool = self::ssr::db_pool();

    Ok("to-be-implemented".into())
}

#[server(GetPost, "/api", "GetJson")]
pub async fn get_post(id: String) -> Result<Post, ServerFnError> {
    // TODO: SQL to select the row
    Ok(Post {
        id: "1".into(),
        dt: Local::now().naive_local(),
        image_url: "https://bit.ly/3t0bA61".into(),
        title: "Some title 1".into(),
        content: "Some content 1".into(),
    })
}

#[server(GetPreviews, "/api", "GetJson")]
pub async fn get_previews(
    oldest: Option<String>,
    newest: Option<String>,
    preview_length: u8,
    page_size: u8,
) -> Result<Vec<Post>, ServerFnError> {
    Ok(vec![
        Post {
            id: "1".into(),
            dt: Local::now().naive_local(),
            image_url: "https://bit.ly/3t0bA61".into(),
            title: "Some title 1".into(),
            content: "Some content 1".into(),
        },
        Post {
            id: "2".into(),
            dt: Local::now().naive_local(),
            image_url: "https://bit.ly/3t0bA61".into(),
            title: "Some title 2".into(),
            content: "Some content 2".into(),
        },
    ])
}
