use super::init_db;

#[tokio::test]
async fn model_db_init_db() -> Result<(), Box<dyn std::error::Error>> {
    // Prepare
    let db = init_db().await?;

    // Evaluate
    let rs = sqlx::query("SELECT * FROM todos").fetch_all(&db).await?;
    assert_eq!(2, rs.len());

    Ok(())
}
