//! WWE Universe Manager - Backend Library
//!
//! This crate provides the backend functionality for the WWE Universe Manager application,
//! including database operations, authentication, and Tauri command handlers.

pub mod auth;
pub mod db;
pub mod models;
pub mod schema;
pub mod types;

use db::{establish_connection, DbState};

/// Main entry point for the Tauri application
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Initialize database connection pool
    let pool = establish_connection();
    let db_state = DbState { pool };

    // Build and run the Tauri application
    tauri::Builder::default()
        .manage(db_state)
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            // Database operations
            db::get_shows,
            db::create_show,
            db::create_user,
            db::create_wrestler,
            db::create_belt,
            // Authentication operations
            auth::verify_credentials,
            auth::register_user,
        ])
        .run(tauri::generate_context!())
        .expect("Failed to run Tauri application");
}
