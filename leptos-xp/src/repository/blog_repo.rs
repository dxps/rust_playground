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

    pub async fn db() -> Result<SqliteConnection, ServerFnError> {
        Ok(SqliteConnection::connect("sqlite:post.db").await?)
    }

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
    let db_conn = self::ssr::db_pool().await?.acquire();

    Ok("to-be-implemented".into())
}

#[server(GetPost, "/api", "GetJson")]
pub async fn get_post(id: String) -> Result<Post, ServerFnError> {
    //
    // let db_conn = self::ssr::db_pool().await?.acquire().await?;
    let mut db_conn = self::ssr::db().await?;
    // let res: Post = sqlx::query_as("SELECT * FROM post WHERE id = ?")
    //     .bind(id)
    //     .fetch_one(&mut db_conn)
    //     .await
    //     .map_err(|_| ServerFnError::ServerError("error getting post".to_string()))?;

    // Ok(res)

    match sqlx::query_as::<_, Post>("SELECT * FROM post WHERE id = ?")
        .bind(id)
        .fetch_one(&mut db_conn)
        .await
    {
        Ok(post) => Ok(post),
        Err(e) => Err(ServerFnError::ServerError(e.to_string())),
    }

    // Ok(Post {
    //     id: "1".into(),
    //     dt: Local::now().naive_local(),
    //     image_url: "https://bit.ly/3t0bA61".into(),
    //     title: "Some title 1".into(),
    //     content: "Some content 1".into(),
    // })
}

#[server(GetPreviews, "/api", "GetJson")]
pub async fn get_previews(
    oldest: Option<String>,
    newest: Option<String>,
    preview_length: u8,
    page_size: u8,
) -> Result<Vec<Post>, ServerFnError> {
    //
    // let db_conn = self::ssr::db_pool().await?.acquire().await?;
    let mut db_conn = self::ssr::db().await?;
    let res: Vec<Post> = sqlx::query_as(
        "SELECT id, dt, image_url, title
         CASE
            WHEN LENGTH(text) > $1 THEN SUBSTR(text, $1) || ' ...'
         END as text
         FROM posts
         ORDER BY dt DESC
         LIMIT $2",
    )
    .bind(preview_length)
    .bind(page_size)
    .fetch_all(&mut db_conn)
    .await?;

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
