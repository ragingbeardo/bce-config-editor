// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod data;

use data::Config;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn testing_paths() -> String {
    let exe_path = std::env::current_exe().unwrap();
    let parent_path = exe_path.parent().unwrap().to_str().unwrap();
    return String::from(parent_path);
}

#[tauri::command]
fn test_write() -> () {
    let exe_path = std::env::current_exe().unwrap();
    let parent_path = exe_path.parent().unwrap().to_str().unwrap();
    let dest_path = String::from(parent_path) + "\\config\\config.json";
    let dest_path_two = String::from(parent_path) + "\\config\\config2.json";

    let pulled_config_string = std::fs::read_to_string(&dest_path);
    let read_result = serde_json::from_str::<Config>(&pulled_config_string.unwrap()).unwrap();


    let _result = std::fs::write(
        dest_path_two,
        serde_json::to_string_pretty(&read_result).unwrap(),
    );
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, testing_paths, test_write])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
