use crate::db::establish_connection;
use crate::models::{NewUser, User};
use crate::schema::users::dsl::*;
use diesel::prelude::*;
use log::{error, info, warn};

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

#[tauri::command]
pub fn verify_credentials(susername: String, spassword: String) -> bool {
    info!("Verifying credentials...");
    let conn = &mut establish_connection();

    match users
        .filter(username.eq(&susername))
        .first::<User>(conn)
        .optional()
    {
        Ok(Some(user)) => {
            if user.password == spassword {
                info!("Credentials verified");
                true
            } else {
                warn!("Invalid password");
                false
            }
        }
        Ok(None) => {
            warn!("User not found");
            false
        }
        Err(err) => {
            error!("Error loading user: {:?}", err);
            false
        }
    }
}

#[tauri::command]
pub fn register_user(susername: String, spassword: String) -> bool {
    let conn = &mut establish_connection();

    if check_user_exists(conn, &susername) {
        warn!("User already exists");
        return false;
    }

    let new_user = NewUser {
        username: &susername,
        password: &spassword,
    };

    match crate::db::create_user(conn, new_user) {
        Some(_) => true,
        None => {
            error!("Failed to create user");
            false
        }
    }
}
