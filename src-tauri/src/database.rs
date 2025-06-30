use anyhow::Result;
use serde::{Deserialize, Serialize};
use sqlx::{migrate::MigrateDatabase, FromRow, Sqlite, SqlitePool};

pub type DbPool = SqlitePool;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Conversation {
    pub id: i64,
    pub title: String,
    pub created_at: String,
    pub llm_provider: String,
    #[sqlx(skip)] // This field is not in the DB, it's populated manually
    #[serde(default)]
    pub messages: Vec<Message>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Message {
    pub id: i64,
    pub conversation_id: i64,
    pub role: String,
    pub content: String,
    pub created_at: String,
}

pub async fn init_db(db_url: &str) -> Result<DbPool> {
    println!("Attempting to initialize database at: {}", db_url);
    if !Sqlite::database_exists(db_url).await.unwrap_or(false) {
        println!("Database does not exist, creating it...");
        Sqlite::create_database(db_url).await?;
        println!("Database created successfully.");
    } else {
        println!("Database already exists.");
    }

    let pool = SqlitePool::connect(db_url).await?;
    println!("Running migrations...");
    sqlx::migrate!("./migrations").run(&pool).await?;
    println!("Migrations completed.");

    Ok(pool)
}

pub async fn create_conversation(
    pool: &DbPool,
    title: &str,
    llm_provider: &str,
) -> Result<Conversation> {
    let mut transaction = pool.begin().await?;
    let conversation_id = sqlx::query!(
        "INSERT INTO conversations (title, llm_provider) VALUES (?, ?)",
        title,
        llm_provider
    )
    .execute(&mut *transaction)
    .await?
    .last_insert_rowid();

    // Use a type that can be either a Pool or a Transaction
    let mut conversation = get_conversation_internal(pool, conversation_id).await?;
    transaction.commit().await?;
    
    conversation.messages = Vec::new();

    Ok(conversation)
}

// An internal function to get a conversation without its messages
async fn get_conversation_internal(pool: &DbPool, id: i64) -> Result<Conversation> {
    Ok(sqlx::query_as::<_, Conversation>("SELECT * FROM conversations WHERE id = ?")
        .bind(id)
        .fetch_one(pool)
        .await?)
}

pub async fn get_conversation(pool: &DbPool, id: i64) -> Result<Conversation> {
    let mut conversation = get_conversation_internal(pool, id).await?;
    conversation.messages = get_messages_for_conversation(pool, id).await?;
    Ok(conversation)
}

pub async fn get_all_conversations(pool: &DbPool) -> Result<Vec<Conversation>> {
    let mut conversations =
        sqlx::query_as::<_, Conversation>("SELECT * FROM conversations ORDER BY created_at DESC")
            .fetch_all(pool)
            .await?;

    for conv in &mut conversations {
        conv.messages = get_messages_for_conversation(pool, conv.id).await?;
    }

    Ok(conversations)
}

pub async fn add_message(
    pool: &DbPool,
    conversation_id: i64,
    role: &str,
    content: &str,
) -> Result<Message> {
    let mut transaction = pool.begin().await?;

    let message_id = sqlx::query!(
        "INSERT INTO messages (conversation_id, role, content) VALUES (?, ?, ?)",
        conversation_id,
        role,
        content
    )
    .execute(&mut *transaction)
    .await?
    .last_insert_rowid();

    let message = get_message(pool, message_id).await?;
    transaction.commit().await?;
    Ok(message)
}

pub async fn get_message(pool: &DbPool, id: i64) -> Result<Message> {
    Ok(sqlx::query_as::<_, Message>("SELECT * FROM messages WHERE id = ?")
        .bind(id)
        .fetch_one(pool)
        .await?)
}

pub async fn get_messages_for_conversation(
    pool: &DbPool,
    conversation_id: i64,
) -> Result<Vec<Message>> {
    Ok(sqlx::query_as::<_, Message>(
        "SELECT * FROM messages WHERE conversation_id = ? ORDER BY created_at ASC",
    )
    .bind(conversation_id)
    .fetch_all(pool)
    .await?)
}

pub async fn delete_conversation(pool: &DbPool, id: i64) -> Result<()> {
    let mut transaction = pool.begin().await?;
    sqlx::query!("DELETE FROM messages WHERE conversation_id = ?", id)
        .execute(&mut *transaction)
        .await?;
    sqlx::query!("DELETE FROM conversations WHERE id = ?", id)
        .execute(&mut *transaction)
        .await?;
    transaction.commit().await?;
    Ok(())
}
