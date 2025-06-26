use crate::db::{DbConnection, DbState};
use crate::models::{NewUser, User};
use crate::schema::users::dsl::*;
use diesel::prelude::*;
use log::{error, info, warn};
use tauri::State;

/// Checks if a user exists in the database
pub fn check_user_exists(conn: &mut SqliteConnection, username_check: &str) -> bool {
    match users
        .filter(username.eq(username_check))
        .select(id)
        .first::<i32>(conn)
        .optional()
    {
        Ok(Some(_)) => {
            info!("User '{}' exists", username_check);
            true
        }
        Ok(None) => {
            info!("User '{}' does not exist", username_check);
            false
        }
        Err(err) => {
            error!(
                "Error checking user existence for '{}': {:?}",
                username_check, err
            );
            false
        }
    }
}

/// Internal function for verifying credentials - used for testing
pub fn internal_verify_credentials(
    conn: &mut SqliteConnection,
    user_username: &str,
    user_password: &str,
) -> bool {
    match users
        .filter(username.eq(user_username))
        .first::<User>(conn)
        .optional()
    {
        Ok(Some(user)) => {
            if user.password == user_password {
                info!("Credentials verified for user: {}", user_username);
                true
            } else {
                warn!("Invalid password for user: {}", user_username);
                false
            }
        }
        Ok(None) => {
            warn!("User not found: {}", user_username);
            false
        }
        Err(err) => {
            error!("Error loading user '{}': {:?}", user_username, err);
            false
        }
    }
}

/// Gets a database connection from the pool
fn get_connection(state: &State<'_, DbState>) -> Result<DbConnection, bool> {
    state.pool.get().map_err(|e| {
        error!("Failed to get connection from pool: {}", e);
        false
    })
}

/// Verifies user credentials via Tauri command
#[tauri::command]
pub fn verify_credentials(
    state: State<'_, DbState>,
    user_username: String,
    user_password: String,
) -> bool {
    info!("Verifying credentials for user: {}", user_username);

    let mut conn = match get_connection(&state) {
        Ok(conn) => conn,
        Err(result) => return result,
    };

    internal_verify_credentials(&mut conn, &user_username, &user_password)
}

/// Registers a new user via Tauri command
#[tauri::command]
pub fn register_user(
    state: State<'_, DbState>,
    user_username: String,
    user_password: String,
) -> bool {
    info!("Attempting to register user: {}", user_username);

    let mut conn = match get_connection(&state) {
        Ok(conn) => conn,
        Err(result) => return result,
    };

    // Check if user already exists
    if check_user_exists(&mut conn, &user_username) {
        warn!(
            "Registration failed: User '{}' already exists",
            user_username
        );
        return false;
    }

    // Create new user
    let new_user = NewUser {
        username: user_username.clone(),
        password: user_password,
    };

    match diesel::insert_into(users)
        .values(&new_user)
        .execute(&mut conn)
    {
        Ok(_) => {
            info!("User '{}' registered successfully", user_username);
            true
        }
        Err(e) => {
            error!("Failed to register user '{}': {}", user_username, e);
            false
        }
    }
}
