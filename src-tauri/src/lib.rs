// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::env;

mod database;
mod llm;
mod crypto_manager;

use database::{
    DbPool, Conversation, Message, create_conversation, get_conversations, add_message, delete_conversation, LlmModel, get_available_models, get_default_model, set_default_model
};
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
    let api_key = get_api_key(app.clone()).await?;
    
    // Get the DbPool state
    let pool = app.state::<DbPool>();

    // Get the default model from the database
    let default_llm_model = database::get_default_model(pool).await?;

    let llm_provider = OpenAIProvider::new(api_key, default_llm_model.model_name);
    llm_provider.chat(messages).await
}

// Main App Setup

async fn run_async() {
    let builder = tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_conversations,
            database::create_conversation,
            database::delete_conversation,
            database::add_message,
            chat_with_llm,
            set_api_key,
            get_api_key,
            database::get_available_models,
            database::get_default_model,
            database::set_default_model
        ]);

    let app = builder
        .build(tauri::generate_context!())
        .expect("error while building tauri application");

    let db_url = resolve_database_path(&app.handle());
    println!("Using database URL: {}", db_url);

    let db_pool = database::init_db(&db_url)
        .await
        .expect("Failed to initialize database");

    app.manage(db_pool);
    app.run(|_app_handle, _event| {});
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Encapsulate async logic in a tokio runtime
    tokio::runtime::Runtime::new()
        .unwrap()
                .block_on(run_async());
}

fn resolve_database_path(app: &tauri::AppHandle) -> String {
    let db_name = "db.sqlite";
    let db_path = if cfg!(debug_assertions) {
        // Development: use the data directory in the project root
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        let project_root = std::path::Path::new(manifest_dir).parent().unwrap();
        let data_dir = project_root.join("data");
        if !data_dir.exists() {
            std::fs::create_dir_all(&data_dir).expect("Failed to create data directory");
        }
        data_dir.join(db_name)
    } else {
        // Production: use the app's data directory
        let app_data_dir = app.path().app_data_dir().expect("Failed to get app data dir");
        if !app_data_dir.exists() {
            std::fs::create_dir_all(&app_data_dir).expect("Failed to create app data directory");
        }
        app_data_dir.join(db_name)
    };

    format!("sqlite:{}", db_path.to_str().unwrap())
}