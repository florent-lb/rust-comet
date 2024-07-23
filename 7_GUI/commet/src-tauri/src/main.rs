// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use commet::service;

fn main() {

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![service::account_service::init_accounts])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
