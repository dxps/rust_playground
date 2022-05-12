use super::init_db;

#[tokio::test]
async fn model_db_init_db() -> Result<(), Box<dyn std::error::Error>> {
    let db_pool = init_db().await?;
    Ok(())
}
