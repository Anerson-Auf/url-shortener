use sqlx::PgPool;
use anyhow::Result;

pub async fn migrate() -> Result<()> {
    dotenvy::dotenv().ok();
    let database = std::env::var("DATABASE_URL")?;
    let pool = PgPool::connect(&database).await?;
    sqlx::migrate!("src/migrations").run(&pool).await?;
    Ok(())
}