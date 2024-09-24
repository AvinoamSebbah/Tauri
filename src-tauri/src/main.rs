#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod user;
mod types;
mod database;

use crate::database::*;
use crate::user::*;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_current_user,
            register_to_waiting_list,
            get_users_waiting_list,
            validate_user
        ])
        .setup(|_app| {
            initialize_db();
            initialize_user();
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
