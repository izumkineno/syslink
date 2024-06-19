// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod database;
mod symlink;
mod commands;
mod utils;

use commands::{
    link,
    read_sled_from_db,
    read_sled_files_from_db,
    remove_sled_from_db
};

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            link,
            read_sled_from_db,
            read_sled_files_from_db,
            remove_sled_from_db
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
