mod files;
mod llm;
mod db;

use tauri::command;
use log::info;
use simple_logger;
use llm::ai::call_neuro;
use db::{client::{get_user_id, get_user_session_data}, 
    models::User, 
    ops::{create_user, create_vault, get_user, get_vaults}
};
use bcrypt::{hash, DEFAULT_COST};

#[command]
fn get_app_version(app: tauri::AppHandle) -> String {
    app.package_info().version.to_string()
}

#[command]
fn logger(message: String) {
    info!("Log: {}", message);
}

#[command]
async fn neuro(prompt: String) -> Result<String, String> {
    call_neuro(&prompt)
    .await
    .map_err(|e| e.to_string())
}

#[command]
async fn add_user(forename: String, email: String, password: String) -> Result<String, String> {
    let hashed_pass = hash(password, DEFAULT_COST).map_err(|e| e.to_string())?;
    create_user(forename, email, hashed_pass)
    .await
    .map_err(|e| e.to_string())
}

#[command]
async fn verify_user(email: String, password: String) -> Result<String, String> {
    get_user(email, password)
        .await
        .map_err(|e| e.to_string())
}


#[command]
async fn get_user_data(session_token: String) -> Result<User, String> {
    get_user_session_data(session_token)
        .await
        .map_err(|e| e.to_string())
}

#[command]
async fn get_id(email: String) -> Result<String, String> {
    get_user_id(email)
        .await
        .map_err(|e| e.to_string())
}

#[command]
async fn add_vault(name: String, id: String) -> Result<String, String> {
    create_vault(name, id)
        .await
        .map_err(|e| e.to_string())
}

#[command]
async fn get_vault_names(id: String) -> Result<Vec<String>, String> {
    get_vaults(id)
        .await
        .map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    simple_logger::init().unwrap();

    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            get_app_version, 
            logger,
            add_user,
            verify_user,
            get_user_data,
            get_id,
            add_vault,
            get_vault_names,
            files::read_file,
            files::save_file,
            files::delete_file,
            files::create_file,
            files::delete_folder,
            files::create_folder,
            files::get_first_file,
            neuro
            ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
