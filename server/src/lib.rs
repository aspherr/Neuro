mod files;
mod llm;
mod db;

use log::info;
use simple_logger;
use llm::ai::call_neuro;
use db::ops::create_user;

#[tauri::command]
fn get_app_version(app: tauri::AppHandle) -> String {
    app.package_info().version.to_string()
}

#[tauri::command]
fn logger(message: String) {
    info!("Log: {}", message);
}

#[tauri::command]
async fn neuro(prompt: String) -> Result<String, String> {
    call_neuro(&prompt)
    .await
    .map_err(|e| e.to_string())
}

#[tauri::command]
async fn add_user(forename: String, email: String, password: String) -> Result<String, String> {
    create_user(forename, email, password)
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
