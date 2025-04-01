pub mod auth;
pub mod db;
pub mod models;
pub mod schema;

use db::{establish_connection, DbState};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let pool = establish_connection();
    let db_state = DbState { pool };

    tauri::Builder::default()
        .manage(db_state)
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            db::get_shows,
            db::create_show,
            db::create_user,
            db::create_wrestler,
            db::create_belt,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
