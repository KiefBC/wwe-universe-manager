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
            db::get_promotions,
            db::create_promotion,
            db::get_shows,
            db::create_show,
            db::get_wrestlers,
            db::get_wrestler_by_id,
            db::update_wrestler_promotion,
            db::update_wrestler_power_ratings,
            db::update_wrestler_basic_stats,
            db::update_wrestler_name,
            db::update_wrestler_real_name,
            db::update_wrestler_biography,
            db::create_user,
            db::create_wrestler,
            db::create_user_wrestler,
            db::create_belt,
            db::get_titles,
            db::get_titles_for_show,
            db::get_unassigned_titles,
            db::update_title_holder,
            db::create_test_data,
            // Show roster operations
            db::get_wrestlers_for_show,
            db::assign_wrestler_to_show,
            db::remove_wrestler_from_show,
            // Match booking operations
            db::create_match,
            db::get_matches_for_show,
            db::add_wrestler_to_match,
            db::get_match_participants,
            db::set_match_winner,
            // Authentication operations
            auth::verify_credentials,
            auth::register_user,
            // Window operations
            open_wrestler_window,
            open_title_window,
        ])
        .run(tauri::generate_context!())
        .expect("Failed to run Tauri application");
}

/// Opens a wrestler details window (only one allowed at a time)
#[tauri::command]
async fn open_wrestler_window(app: AppHandle, wrestler_id: Option<String>) -> Result<(), String> {
    let wrestler_id = wrestler_id.unwrap_or_else(|| "default".to_string());
    let window_label = "wrestler-details"; // Use consistent label for all wrestler windows
    
    // Check if window already exists
    if let Some(existing_window) = app.get_webview_window(window_label) {
        // If window exists, update the URL hash to load the new wrestler
        let js_code = format!("window.location.hash = '#wrestler?id={}';", wrestler_id);
        existing_window.eval(&js_code)
            .map_err(|e| e.to_string())?;
        existing_window.set_focus().map_err(|e| e.to_string())?;
        return Ok(());
    }

    // Create new window with wrestler ID in the URL hash
    let url = format!("index.html#wrestler?id={}", wrestler_id);
    let _window = tauri::WebviewWindowBuilder::new(
        &app,
        window_label,
        tauri::WebviewUrl::App(url.into()),
    )
    .title("Wrestler Details")
    .inner_size(885.0, 860.0)
    .min_inner_size(600.0, 500.0)
    .center()
    .build()
    .map_err(|e| e.to_string())?;

    Ok(())
}

/// Opens a title details window (only one allowed at a time)
#[tauri::command]
async fn open_title_window(app: AppHandle, title_id: Option<String>) -> Result<(), String> {
    let title_id = title_id.unwrap_or_else(|| "default".to_string());
    let window_label = "title-details"; // Use consistent label for all title windows
    
    // Check if window already exists
    if let Some(existing_window) = app.get_webview_window(window_label) {
        // If window exists, update the URL hash to load the new title
        let js_code = format!("window.location.hash = '#title?id={}';", title_id);
        existing_window.eval(&js_code)
            .map_err(|e| e.to_string())?;
        existing_window.set_focus().map_err(|e| e.to_string())?;
        return Ok(());
    }

    // Create new window with title ID in the URL hash
    let url = format!("index.html#title?id={}", title_id);
    let _window = tauri::WebviewWindowBuilder::new(
        &app,
        window_label,
        tauri::WebviewUrl::App(url.into()),
    )
    .title("Title Details")
    .inner_size(885.0, 860.0)
    .min_inner_size(600.0, 500.0)
    .center()
    .build()
    .map_err(|e| e.to_string())?;

    Ok(())
}
