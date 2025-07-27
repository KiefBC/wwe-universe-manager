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
use tauri::{AppHandle, Manager};

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
            // Window operations
            open_wrestler_window,
        ])
        .run(tauri::generate_context!())
        .expect("Failed to run Tauri application");
}

/// Opens a new window displaying wrestler details
#[tauri::command]
async fn open_wrestler_window(app: AppHandle, wrestler_id: Option<String>) -> Result<(), String> {
    let window_label = format!("wrestler-{}", wrestler_id.unwrap_or_else(|| "default".to_string()));
    
    // Check if window already exists
    if let Some(_existing_window) = app.get_webview_window(&window_label) {
        // If window exists, just focus it
        if let Some(window) = app.get_webview_window(&window_label) {
            window.set_focus().map_err(|e| e.to_string())?;
            return Ok(());
        }
    }

    // Create new window
    let _window = tauri::WebviewWindowBuilder::new(
        &app,
        window_label,
        tauri::WebviewUrl::App("index.html#wrestler".into()),
    )
    .title("Wrestler Details")
    .inner_size(900.0, 700.0)
    .min_inner_size(600.0, 500.0)
    .center()
    .build()
    .map_err(|e| e.to_string())?;

    Ok(())
}
