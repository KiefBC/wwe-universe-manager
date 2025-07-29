use crate::db::{DbConnection, DbState};
use crate::models::{NewUser, User};
use crate::schema::users::dsl::*;
use diesel::prelude::*;
use log::{error, info, warn};
use tauri::State;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use argon2::password_hash::SaltString;
use rand::rngs::OsRng;

/// Hash a password using Argon2
fn hash_password(password_str: &str) -> Result<String, String> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    
    argon2
        .hash_password(password_str.as_bytes(), &salt)
        .map(|hash| hash.to_string())
        .map_err(|e| format!("Failed to hash password: {}", e))
}

/// Verify a password against its hash
fn verify_password(password_str: &str, hash_str: &str) -> Result<bool, String> {
    let parsed_hash = PasswordHash::new(hash_str)
        .map_err(|e| format!("Failed to parse password hash: {}", e))?;
    
    let argon2 = Argon2::default();
    
    match argon2.verify_password(password_str.as_bytes(), &parsed_hash) {
        Ok(()) => Ok(true),
        Err(_) => Ok(false),
    }
}

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
            match verify_password(user_password, &user.password) {
                Ok(true) => {
                    info!("Credentials verified for user: {}", user_username);
                    true
                }
                Ok(false) => {
                    warn!("Invalid password for user: {}", user_username);
                    false
                }
                Err(e) => {
                    error!("Password verification error for user '{}': {}", user_username, e);
                    false
                }
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

    // Hash the password before storing
    let hashed_password = match hash_password(&user_password) {
        Ok(hash) => hash,
        Err(e) => {
            error!("Failed to hash password for user '{}': {}", user_username, e);
            return false;
        }
    };

    // Create new user with hashed password
    let new_user = NewUser {
        username: user_username.clone(),
        password: hashed_password,
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
