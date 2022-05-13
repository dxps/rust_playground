use super::db::DbPool;

#[derive(sqlx::FromRow, Clone, Debug)]
pub struct Todo {
    pub id: i64,
    pub cid: i64, // creator id
    pub title: String,
}

// `MAC` stands for model access controller.
pub struct TodoMAC;

impl TodoMAC {
    pub async fn list(db: &DbPool) -> Result<Vec<Todo>, sqlx::Error> {
        let sql = "SELECT id, cid, title FROM todos ORDER by id DESC";
        let query = sqlx::query_as(&sql);
        let todos = query.fetch_all(db).await?;
        Ok(todos)
    }
}

#[cfg(test)]
#[path = "../zz_tests/model_todo.rs"]
mod tests;
