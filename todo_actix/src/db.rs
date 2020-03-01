use std::io;

use deadpool_postgres::{Client, Pool};
use tokio_pg_mapper::FromTokioPostgresRow;

use crate::models::TodoList;

pub async fn get_todos(client: &Client) -> Result<Vec<TodoList>, io::Error> {
    let stmt = client.prepare("select * from todo_list").await.unwrap();

    let todos = client
        .query(&stmt, &[])
        .await
        .expect("Error getting todo lists")
        .iter()
        .map(|row| TodoList::from_row_ref(row).unwrap())
        .collect::<Vec<TodoList>>();

    Ok(todos)
}

/// This function checks if a database connection can be retrieved from the pool.
/// By default, the pool can be initialized no matter the database config or availability.
pub async fn check_init_db_conn(pool: Pool) -> bool {
    match pool.get().await {
        Err(err) => {
            println!(">>> Error getting DB Connection: '{}'", err);
            false
        }
        Ok(_) => true,
    }
}
