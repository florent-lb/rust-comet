// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use commet::service;
use service::account_service;

fn main() {

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![account_service::init_accounts,account_service::get_all_accounts,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
