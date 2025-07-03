// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod database;
mod llm;
mod crypto_manager;

use database::{DbPool, Conversation, Message, create_conversation, get_conversations, add_message, delete_conversation};
use llm::{LLM, OpenAIProvider};
use tauri::Manager;

// Tauri Commands

#[tauri::command]
async fn set_api_key(app: tauri::AppHandle, api_key: String) -> Result<(), String> {
    let app_data_dir = app.path().app_data_dir().expect("Failed to get app data dir");
    crypto_manager::encrypt_and_save_api_key(app_data_dir, &api_key)
}

#[tauri::command]
async fn get_api_key(app: tauri::AppHandle) -> Result<String, String> {
    let app_data_dir = app.path().app_data_dir().expect("Failed to get app data dir");
    crypto_manager::decrypt_api_key(app_data_dir)
}

#[tauri::command]
async fn chat_with_llm(
    messages: Vec<llm::Message>,
    app: tauri::AppHandle,
) -> Result<llm::Message, String> {
    let api_key = get_api_key(app).await?;
    let llm_provider = OpenAIProvider::new(api_key);
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

    tauri::Builder::default()
        .manage(db_pool)
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_conversations,
            database::create_conversation,
            database::delete_conversation,
            database::add_message,
            chat_with_llm,
            set_api_key,
            get_api_key
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