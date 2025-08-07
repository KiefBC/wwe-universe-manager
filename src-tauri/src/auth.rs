//! Authentication module for user management and security
//! 
//! This module provides password hashing, verification, and user registration
//! functionality using Argon2 for secure password storage.
//!
//! **Note**: Currently unused in application (no user account creation flow),
//! but fully implemented and ready for future use.

use crate::db::{DbConnection, DbState};
use crate::models::{NewUser, User};
use crate::schema::users::dsl::*;
use diesel::prelude::*;
use log::{error, info, warn};
use tauri::State;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use argon2::password_hash::SaltString;
use rand::rngs::OsRng;

/// Hash a password using Argon2 with a random salt
/// 
/// # Arguments
/// * `password_str` - The plain text password to hash
/// 
/// # Returns
/// * `Ok(String)` - The hashed password in PHC string format
/// * `Err(String)` - Error message if hashing fails
/// 
/// # Security
/// Uses Argon2 with default parameters and a random salt
#[allow(dead_code)]
fn hash_password(password_str: &str) -> Result<String, String> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    
    argon2
        .hash_password(password_str.as_bytes(), &salt)
        .map(|hash| hash.to_string())
        .map_err(|e| format!("Failed to hash password: {}", e))
}

/// Verify a password against its Argon2 hash
/// 
/// # Arguments
/// * `password_str` - The plain text password to verify
/// * `hash_str` - The stored password hash in PHC string format
/// 
/// # Returns
/// * `Ok(true)` - Password matches the hash
/// * `Ok(false)` - Password does not match
/// * `Err(String)` - Error message if verification fails
#[allow(dead_code)]
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
/// 
/// # Arguments
/// * `conn` - Mutable reference to the database connection
/// * `username_check` - The username to check
/// 
/// # Returns
/// * `true` - User exists
/// * `false` - User does not exist or error occurred
#[allow(dead_code)]
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

/// Internal function for verifying user credentials
/// 
/// # Arguments
/// * `conn` - Mutable reference to the database connection
/// * `user_username` - The username to verify
/// * `user_password` - The plain text password to verify
/// 
/// # Returns
/// * `true` - Credentials are valid
/// * `false` - Invalid credentials or error occurred
/// 
/// # Note
/// Used by both Tauri commands and tests
#[allow(dead_code)]
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
/// 
/// # Arguments
/// * `state` - The Tauri state containing the database pool
/// 
/// # Returns
/// * `Ok(DbConnection)` - A pooled database connection
/// * `Err(bool)` - Returns false if connection cannot be obtained
#[allow(dead_code)]
fn get_connection(state: &State<'_, DbState>) -> Result<DbConnection, bool> {
    state.pool.get().map_err(|e| {
        error!("Failed to get connection from pool: {}", e);
        false
    })
}

/// Tauri command to verify user login credentials
/// 
/// # Arguments
/// * `state` - The Tauri state containing the database pool
/// * `user_username` - The username to verify
/// * `user_password` - The plain text password to verify
/// 
/// # Returns
/// * `true` - Login successful
/// * `false` - Invalid credentials or error
#[allow(dead_code)]
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

/// Tauri command to register a new user account
/// 
/// # Arguments
/// * `state` - The Tauri state containing the database pool
/// * `user_username` - The desired username
/// * `user_password` - The plain text password (will be hashed)
/// 
/// # Returns
/// * `true` - Registration successful
/// * `false` - Username taken or error occurred
/// 
/// # Security
/// Passwords are hashed using Argon2 before storage
#[allow(dead_code)]
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
