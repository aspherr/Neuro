use log::info;
use simple_logger;

#[tauri::command]
fn get_app_version(app: tauri::AppHandle) -> String {
    app
    .package_info()
    .version
    .to_string()
}

#[tauri::command]
fn logger(message: String) {
    info!("Log: {}", message);
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    simple_logger::init()
    .unwrap();
    
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![get_app_version, logger])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
