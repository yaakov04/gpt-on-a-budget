use anyhow::Result;
use tauri::State;
use serde::{Deserialize, Serialize};
use sqlx::{migrate::MigrateDatabase, FromRow, Sqlite, SqlitePool};
use chrono;

pub type DbPool = SqlitePool;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Conversation {
    pub id: i64,
    pub title: String,
    pub created_at: String,
    pub llm_provider: String,
    #[sqlx(skip)]
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

#[tauri::command]
pub async fn create_conversation(
    pool: State<'_, DbPool>,
    title: &str,
    llm_provider: &str,
) -> Result<Conversation, String> {
    let mut transaction = pool.begin().await.map_err(|e| e.to_string())?;

    let conversation_id = sqlx::query!(
        "INSERT INTO conversations (title, llm_provider) VALUES (?, ?)",
        title,
        llm_provider
    )
    .execute(&mut *transaction)
    .await
    .map_err(|e| e.to_string())?
    .last_insert_rowid();

    let new_conversation = Conversation {
        id: conversation_id,
        title: title.to_string(),
        created_at: chrono::Utc::now().to_rfc3339(),
        llm_provider: llm_provider.to_string(),
        messages: Vec::new(),
    };

    transaction.commit().await.map_err(|e| e.to_string())?;

    Ok(new_conversation)
}

// Helper interno
async fn get_conversation_internal(pool: &DbPool, id: i64) -> Result<Conversation, sqlx::Error> {
    sqlx::query_as::<_, Conversation>("SELECT * FROM conversations WHERE id = ?")
        .bind(id)
        .fetch_one(pool)
        .await
}

#[tauri::command]
pub async fn get_conversations(
    pool: State<'_, DbPool>,
) -> Result<Vec<Conversation>, String> {
    let mut conversations = sqlx::query_as::<_, Conversation>(
        "SELECT * FROM conversations ORDER BY created_at DESC"
    )
    .fetch_all(&*pool)
    .await
    .map_err(|e| e.to_string())?;

    for conv in &mut conversations {
        conv.messages = get_messages_for_conversation(&pool, conv.id).await?;
    }

    Ok(conversations)
}

#[tauri::command]
pub async fn add_message(
    pool: State<'_, DbPool>,
    conversation_id: i64,
    role: &str,
    content: &str,
) -> Result<Message, String> {
    let mut transaction = pool.begin().await.map_err(|e| e.to_string())?;

    let message_id = sqlx::query!(
        "INSERT INTO messages (conversation_id, role, content) VALUES (?, ?, ?)",
        conversation_id,
        role,
        content
    )
    .execute(&mut *transaction)
    .await
    .map_err(|e| e.to_string())?
    .last_insert_rowid();

    let new_message = Message {
        id: message_id,
        conversation_id,
        role: role.to_string(),
        content: content.to_string(),
        created_at: chrono::Utc::now().to_rfc3339(),
    };

    transaction.commit().await.map_err(|e| e.to_string())?;

    Ok(new_message)
}

pub async fn get_messages_for_conversation(
    pool: &DbPool,
    conversation_id: i64,
) -> Result<Vec<Message>, String> {
    let result = sqlx::query_as::<_, Message>(
        "SELECT * FROM messages WHERE conversation_id = ? ORDER BY created_at ASC",
    )
    .bind(conversation_id)
    .fetch_all(pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(result)
}

#[tauri::command]
pub async fn delete_conversation(
    pool: State<'_, DbPool>,
    id: i64,
) -> Result<(), String> {
    let mut transaction = pool.begin().await.map_err(|e| e.to_string())?;

    sqlx::query!("DELETE FROM messages WHERE conversation_id = ?", id)
        .execute(&mut *transaction)
        .await
        .map_err(|e| e.to_string())?;

    sqlx::query!("DELETE FROM conversations WHERE id = ?", id)
        .execute(&mut *transaction)
        .await
        .map_err(|e| e.to_string())?;

    transaction.commit().await.map_err(|e| e.to_string())?;

    Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct LlmModel {
    pub id: i64,
    pub model_name: String,
    pub provider: String,
    pub is_default: bool,
}

#[tauri::command]
pub async fn get_available_models(pool: State<'_, DbPool>) -> Result<Vec<LlmModel>, String> {
    sqlx::query_as::<_, LlmModel>("SELECT * FROM llm_models ORDER BY provider, model_name")
        .fetch_all(&*pool)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_default_model(pool: State<'_, DbPool>) -> Result<LlmModel, String> {
    sqlx::query_as::<_, LlmModel>("SELECT * FROM llm_models WHERE is_default = 1 LIMIT 1")
        .fetch_one(&*pool)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn set_default_model(pool: State<'_, DbPool>, model_id: i64) -> Result<(), String> {
    let mut transaction = pool.begin().await.map_err(|e| e.to_string())?;

    // Reset the current default
    sqlx::query("UPDATE llm_models SET is_default = 0")
        .execute(&mut *transaction)
        .await
        .map_err(|e| e.to_string())?;

    // Set the new default
    sqlx::query("UPDATE llm_models SET is_default = 1 WHERE id = ?")
        .bind(model_id)
        .execute(&mut *transaction)
        .await
        .map_err(|e| e.to_string())?;

    transaction.commit().await.map_err(|e| e.to_string())?;

    Ok(())
}
