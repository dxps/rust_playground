use std::io;

use deadpool_postgres::{Client, Pool};
use tokio_pg_mapper::FromTokioPostgresRow;

use crate::models::{TodoItem, TodoList};

/// Get all todolists that exist.
pub async fn get_todos(client: &Client) -> Result<Vec<TodoList>, io::Error> {
    let stmt = client
        .prepare("select * from todo_list order by id desc")
        .await
        .unwrap();
    let todos = client
        .query(&stmt, &[])
        .await
        .expect("Error getting todo lists")
        .iter()
        .map(|row| TodoList::from_row_ref(row).unwrap())
        .collect::<Vec<TodoList>>();

    Ok(todos)
}

/// Get the items that exists in a todolist.
pub async fn get_todo_items(client: &Client, id: i32) -> Result<Vec<TodoItem>, io::Error> {
    let stmt = client
        .prepare("select * from todo_item where list_id = $1")
        .await
        .unwrap();
    let items = client
        .query(&stmt, &[&id])
        .await
        .expect("Error getting items")
        .iter()
        .map(|row| TodoItem::from_row_ref(row).unwrap())
        .collect::<Vec<TodoItem>>();

    Ok(items)
}

/// This function checks if a database connection can be retrieved from the pool.
/// As by default, the pool gets initialized without any connection test, this is called at startup.
pub async fn check_init_db_conn(pool: Pool) -> bool {
    match pool.get().await {
        Err(err) => {
            println!(">>> Error getting database connection: '{}'", err);
            false
        }
        Ok(_) => true,
    }
}

pub async fn create_todo(client: &Client, title: String) -> Result<TodoList, io::Error> {
    let stmt = client
        .prepare("insert into todo_list (title) values ($1) returning id, title")
        .await
        .unwrap();
    client
        .query(&stmt, &[&title])
        .await
        .expect("Error creating todo list")
        .iter()
        .map(|row| TodoList::from_row_ref(row).unwrap())
        .collect::<Vec<TodoList>>()
        .pop()
        .ok_or(io::Error::new(
            io::ErrorKind::Other,
            "Error creating todo list",
        ))
}
