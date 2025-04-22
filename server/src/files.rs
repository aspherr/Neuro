use std::fs;
use std::path::PathBuf;
use tauri::command;

// reads file content
#[command]
pub fn read_file(path: &str) -> Result<String, String> {
    fs::read_to_string(path).map_err(|e| e.to_string())
}

// writes file content
#[command]
pub fn save_file(path: &str, data: &str) -> Result<(), String> {
    fs::write(path, data).map_err(|e| e.to_string())
}

// deletes file
#[command]
pub fn delete_file(path: &str) -> Result<(), String> {
    fs::remove_file(path).map_err(|e| e.to_string())
}

// creates file
#[command]
pub fn create_file(path: &str) -> Result<(), String> {
    fs::File::create(path).map_err(|e| e.to_string())?;
    Ok(())
}

// deletes folder/directory
#[command]
pub fn delete_folder(path: &str) -> Result<(), String> {
    fs::remove_dir_all(path).map_err(|e| e.to_string())
}

// create folder/directory
#[command]
pub fn create_folder(path: &str) -> Result<(), String> {
    fs::create_dir_all(path).map_err(|e| e.to_string())?;
    Ok(())
}

// grab the first file within a specified folder
#[command]
pub fn get_first_file(path: &str) -> Option<PathBuf> {
    let mut dir = fs::read_dir(path).ok()?;
    let file = dir.next()?.ok()?;
    let path = file.path();
    
    if path.is_file() {
        return Some(path);
    
    } else {
        None
    }
} 