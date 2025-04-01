use crate::db::DbState;
use crate::models::{NewUser, User};
use crate::schema::users::dsl::*;
use diesel::prelude::*;
use log::{error, info, warn};
use tauri::State;

pub fn check_user_exists(conn: &mut SqliteConnection, username_check: &str) -> bool {
    match users
        .filter(username.eq(username_check))
        .select(id)
        .first::<i32>(conn)
        .optional()
    {
        Ok(Some(_)) => {
            warn!("User exists");
            true
        }
        Ok(None) => false,
        Err(err) => {
            error!("Error checking user existence: {:?}", err);
            false
        }
    }
}

// Internal function for verifying credentials, testable directly
pub fn internal_verify_credentials(conn: &mut SqliteConnection, susername: &str, spassword: &str) -> bool {
    match users
        .filter(username.eq(susername))
        .first::<User>(conn)
        .optional()
    {
        Ok(Some(user)) => {
            if user.password == spassword {
                info!("Credentials verified for user: {}", susername);
                true
            } else {
                warn!("Invalid password for user: {}", susername);
                false
            }
        }
        Ok(None) => {
            warn!("User not found: {}", susername);
            false
        }
        Err(err) => {
            error!("Error loading user {}: {:?}", susername, err);
            false
        }
    }
}

#[tauri::command]
pub fn verify_credentials(state: State<'_, DbState>, susername: String, spassword: String) -> bool {
    info!("Verifying credentials for user: {}...", susername);
    let mut conn = match state.pool.get() {
        Ok(conn) => conn,
        Err(e) => {
            error!("Failed to get connection from pool: {}", e);
            return false;
        }
    };

    // Call the internal function
    internal_verify_credentials(&mut *conn, &susername, &spassword)
}

#[tauri::command]
pub fn register_user(state: State<'_, DbState>, susername: String, spassword: String) -> bool {
    let mut conn = match state.pool.get() {
        Ok(conn) => conn,
        Err(e) => {
            error!("Failed to get connection from pool: {}", e);
            return false;
        }
    };

    if check_user_exists(&mut *conn, &susername) {
        warn!("User already exists");
        return false;
    }

    let new_user = NewUser {
        username: susername.clone(),
        password: spassword.clone(),
    };

    match diesel::insert_into(users)
        .values(&new_user)
        .execute(&mut *conn)
    {
        Ok(_) => {
            info!("User '{}' created successfully", susername);
            true
        }
        Err(e) => {
            error!("Failed to create user: {}", e);
            false
        }
    }
}
