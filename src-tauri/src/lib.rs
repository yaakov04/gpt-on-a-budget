// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod database;
mod llm;

use database::{DbPool, Conversation, Message};
use llm::{LLM, OpenAIProvider};
use tauri::State;

// Tauri Commands

#[tauri::command]
async fn get_conversations(pool: State<'_, DbPool>) -> Result<Vec<Conversation>, String> {
    database::get_all_conversations(&pool)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn create_conversation(
    pool: State<'_, DbPool>,
    title: String,
) -> Result<Conversation, String> {
    // For now, llm_provider is hardcoded. This will be dynamic in the future.
    database::create_conversation(&pool, &title, "openai")
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn delete_conversation(pool: State<'_, DbPool>, id: i64) -> Result<(), String> {
    database::delete_conversation(&pool, id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn add_message(
    pool: State<'_, DbPool>,
    conversation_id: i64,
    role: String,
    content: String,
) -> Result<Message, String> {
    database::add_message(&pool, conversation_id, &role, &content)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn chat_with_llm(
    messages: Vec<llm::Message>,
    llm_provider: State<'_, OpenAIProvider>,
) -> Result<llm::Message, String> {
    llm_provider.chat(messages).await
}

// Main App Setup

async fn run_async() {
    dotenvy::dotenv().expect("Failed to load .env file");

    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set.");

    println!("Using database URL: {}", db_url);

    let db_pool = database::init_db(&db_url)
        .await
        .expect("Failed to initialize database");

    let openai_api_key = std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set.");
    let openai_provider = OpenAIProvider::new(openai_api_key);

    tauri::Builder::default()
        .manage(db_pool)
        .manage(openai_provider)
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_conversations,
            create_conversation,
            delete_conversation,
            add_message,
            chat_with_llm
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Encapsulate async logic in a tokio runtime
    tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(run_async());
}