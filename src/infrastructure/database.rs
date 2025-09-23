use sqlx::SqlitePool;
use anyhow::Result;

pub async fn create_connection_pool() -> Result<SqlitePool> {
    let database_url = "sqlite:products.db";
    let pool = SqlitePool::connect(database_url).await?;
    Ok(pool)
}

#[allow(dead_code)]
pub async fn run_migrations(pool: &SqlitePool) -> Result<()> {
    // Create tables if they don't exist
    sqlx::query(include_str!("../../migrations/001_create_products.sql"))
        .execute(pool)
        .await?;
    
    Ok(())
}