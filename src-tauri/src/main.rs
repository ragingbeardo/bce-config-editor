// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod data;

use data::Config;

#[tauri::command]
fn read_current_config() -> () {
    let exe_path = std::env::current_exe().unwrap();
    let parent_path = exe_path.parent().unwrap().to_str().unwrap();
    let dest_path = String::from(parent_path) + "\\config\\config.json";

    let pulled_config_string = std::fs::read_to_string(&dest_path);
    let read_result = serde_json::from_str::<Config>(&pulled_config_string.unwrap()).unwrap();


    let _result = std::fs::write(
        dest_path,
        serde_json::to_string_pretty(&read_result).unwrap(),
    );
}

#[tauri::command]
fn write_new_config() -> () {
    let exe_path = std::env::current_exe().unwrap();
    let parent_path = exe_path.parent().unwrap().to_str().unwrap();
    let dest_path = String::from(parent_path) + "\\config\\config.json";

    let pulled_config_string = std::fs::read_to_string(&dest_path);
    let read_result = serde_json::from_str::<Config>(&pulled_config_string.unwrap()).unwrap();


    let _result = std::fs::write(
        dest_path,
        serde_json::to_string_pretty(&read_result).unwrap(),
    );
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![read_current_config, write_new_config])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
