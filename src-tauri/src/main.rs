// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[tauri::command]
fn read_current_config() -> String {
    let exe_path = std::env::current_exe().expect("Failed to get current executable path");
    let parent_path = exe_path.parent().expect("Failed to get parent path").to_str().expect("Failed to convert path to string");
    let dest_path = format!("{}\\config\\config.json", parent_path);

    std::fs::read_to_string(&dest_path).expect("Failed to read config file")
}

#[tauri::command]
fn write_new_config(json_data: String) -> Result<(), String> {
    let exe_path = std::env::current_exe().map_err(|e| e.to_string())?;
    let parent_path = exe_path.parent().ok_or("Failed to get parent path")?.to_str().ok_or("Failed to convert path to string")?;
    let dest_path = format!("{}\\config\\config.json", parent_path);

    std::fs::write(&dest_path, json_data).map_err(|e| e.to_string())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![read_current_config, write_new_config])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
