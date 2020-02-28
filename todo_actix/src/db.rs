use std::io;

use deadpool_postgres::Client;
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
