mod files;
mod llm;
mod db;

use tauri::command;
use simple_logger;
use llm::ai::call_neuro;
use db::{client::{get_user_id, get_user_session_data, delete_session}, 
    models::User, 
    ops::{create_user, create_vault, get_user, get_vaults,  get_vault_id, delete_vault, create_notebook, get_notebooks, get_notebook_id, delete_notebook, create_note, get_notes, get_note_id, read_note, save_note, delete_note}
};
use bcrypt::{hash, DEFAULT_COST};

#[command]
fn get_app_version(app: tauri::AppHandle) -> String {
    app.package_info().version.to_string()
}

/* logger for debugging - comment out to use
use log::info;
#[command]
fn logger(message: String) {
    info!("Log: {}", message);
}
*/

// API call to OpenAI GPT model
#[command]
async fn neuro(prompt: String) -> Result<String, String> {
    call_neuro(&prompt)
    .await
    .map_err(|e| e.to_string())
}

// calls create user query
#[command]
async fn add_user(forename: String, email: String, password: String) -> Result<String, String> {
    let hashed_pass = hash(password, DEFAULT_COST).map_err(|e| e.to_string())?; // encrypts user password before injecting 
    create_user(forename, email, hashed_pass)
    .await
    .map_err(|e| e.to_string())
}

// verifies user account for login
#[command]
async fn verify_user(email: String, password: String) -> Result<String, String> {
    get_user(email, password)
        .await
        .map_err(|e| e.to_string())
}

// calls get session data query
#[command]
async fn get_user_data(session_token: String) -> Result<User, String> {
    get_user_session_data(session_token)
        .await
        .map_err(|e| e.to_string())
}

// calls get user ID query
#[command]
async fn get_id(email: String) -> Result<String, String> {
    get_user_id(email)
        .await
        .map_err(|e| e.to_string())
}

// calls create vault query
#[command]
async fn add_vault(name: String, id: String) -> Result<String, String> {
    create_vault(name, id)
        .await
        .map_err(|e| e.to_string())
}

// calls get vault name query
#[command]
async fn get_vault_names(id: String) -> Result<Vec<String>, String> {
    get_vaults(id)
        .await
        .map_err(|e| e.to_string())
}

// calls get vault ID query
#[command]
async fn vault_id(name: String) -> Result<String, String> {
    get_vault_id(name)
        .await
        .map_err(|e| e.to_string())
}

// calls delete vault query
#[command]
async fn drop_vault(vid: String, name: String, uid: String) -> Result<(), String> {
    delete_vault(vid, name, uid)
        .await
        .map_err(|e| e.to_string())
}

// calls logout query
#[command]
async fn logout(token: String, id: String) -> Result<bool, String> {
    delete_session(token, id)
        .await
        .map_err(|e| e.to_string())
}


// calls create notebook query
#[command]
async fn add_notebook(name: String, id: String) -> Result<String, String> {
    create_notebook(name, id)
        .await
        .map_err(|e| e.to_string())
}

// calls get notebook names query
#[command]
async fn get_notebook_names(id: String) -> Result<Vec<String>, String> {
    get_notebooks(id)
        .await
        .map_err(|e| e.to_string())
}

// calls get notebook ID query
#[command]
async fn notebook_id(name: String) -> Result<String, String> {
    get_notebook_id(name)
        .await
        .map_err(|e| e.to_string())
}

// calls delete notebook query
#[command]
async fn drop_notebook(nid: String, name: String, vid: String) -> Result<(), String> {
    delete_notebook(nid, name, vid)
        .await
        .map_err(|e| e.to_string())
}

// calls add note query
#[command]
async fn add_note(name: String, nid: String) -> Result<String, String> {
    create_note(name, nid)
        .await
        .map_err(|e| e.to_string())
}

// calls get note names query
#[command]
async fn get_note_names(id: String) -> Result<Vec<String>, String> {
    get_notes(id)
        .await
        .map_err(|e| e.to_string())
}

// calls get note ID query
#[command]
async fn note_id(name: String) -> Result<String, String> {
    get_note_id(name)
        .await
        .map_err(|e| e.to_string())
}

// calls read note query
#[command]
async fn read_remote_note(id: String) -> Result<String, String> {
    read_note(id)
        .await
        .map_err(|e| e.to_string())
}

// calls save note query
#[command]
async fn save_remote_note(id: String, content: String) -> Result<String, String> {
    save_note(id, content)
        .await
        .map_err(|e| e.to_string())
}

// calls delete note query
#[command]
async fn delete_remote_note(id: String, name: String, nid: String) -> Result<(), String> {
    delete_note(id, name, nid)
        .await
        .map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    simple_logger::init().unwrap();

    // Tauri API method calls
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            get_app_version, 
            add_user,
            verify_user,
            get_user_data,
            get_id,
            add_vault,
            get_vault_names,
            vault_id,
            drop_vault,
            logout,
            add_notebook,
            get_notebook_names,
            notebook_id,
            drop_notebook,
            add_note,
            get_note_names,
            note_id,
            read_remote_note,
            save_remote_note,
            delete_remote_note,
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
