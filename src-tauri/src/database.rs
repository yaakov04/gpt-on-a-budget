use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};
use std::env;
use anyhow::Result;

pub async fn init_db() -> Result<SqlitePool> {
    let db_url = env::var("DATABASE_URL")?;
    
    if !Sqlite::database_exists(&db_url).await.unwrap_or(false) {
        Sqlite::create_database(&db_url).await?;
    }

    let pool = SqlitePool::connect(&db_url).await?;
    
    // Aquí irán las migraciones
    sqlx::migrate!("./migrations").run(&pool).await?;
    
    Ok(pool)
}
