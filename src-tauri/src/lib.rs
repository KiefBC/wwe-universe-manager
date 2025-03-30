pub mod auth;
pub mod db;
pub mod models;
pub mod schema;

use db::{establish_connection, DbState};
use std::sync::Mutex;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let conn = establish_connection();
    let db_state = DbState {
        db: Mutex::new(conn),
    };

    tauri::Builder::default()
        .manage(db_state)
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet,
            db::get_shows,
            db::create_show,
            db::create_user,
            db::create_wrestler,
            db::create_belt,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
