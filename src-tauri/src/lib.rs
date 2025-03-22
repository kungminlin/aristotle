use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

#[tauri::command]
fn update_file(path: &str, blob: &str) -> Result<(), String> {
    // TODO: Make base path configurable
    let base_path = PathBuf::from("./target/debug/");
    let file_path = base_path.join(path.trim_start_matches('/')); // Remove leading slash if present
    
    let mut file = File::create(&file_path)
        .map_err(|e| format!("Failed to create file: {}", e))?;
    
    file.write_all(blob.as_bytes())
        .map_err(|e| format!("Failed to write to file: {}", e))?;
    
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![update_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
