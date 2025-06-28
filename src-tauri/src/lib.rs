mod database;
mod llm;

use tauri::Manager;
use database::init_db;
use llm::{LLM, OpenAIProvider, Message};

#[tauri::command]
async fn chat_with_llm(messages: Vec<Message>, llm_provider: tauri::State<'_, OpenAIProvider>) -> Result<Message, String> {
    llm_provider.chat(messages).await
}

async fn run_async() {
    dotenvy::dotenv().expect("Failed to load .env file");

    let db_pool = init_db().await.expect("Failed to initialize database");
    let openai_api_key = std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set.");
    let openai_provider = OpenAIProvider::new(openai_api_key);

    tauri::Builder::default()
        .manage(db_pool)
        .manage(openai_provider)
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![chat_with_llm])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(run_async());
}
