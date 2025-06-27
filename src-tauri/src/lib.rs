mod database;

//use tauri::Manager;
use database::init_db;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

async fn run_async() {
    dotenvy::dotenv().expect("Failed to load .env file");

    let db_pool = init_db().await.expect("Failed to initialize database");

    tauri::Builder::default()
        .manage(db_pool)
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(run_async());
}
