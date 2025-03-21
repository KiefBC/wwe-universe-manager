use crate::models::{NewTitle, NewUser, NewWrestler, Title, User, Wrestler};
use diesel::prelude::*;
use dotenvy::dotenv;
use log::{error, info};
use std::env;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok().expect("Error loading .env file");
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let mut conn = SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));

    // Enable foreign key constraints using a raw SQL query
    diesel::sql_query("PRAGMA foreign_keys = ON")
        .execute(&mut conn)
        .expect("Failed to enable foreign key constraints");

    conn
}

#[tauri::command]
pub fn create_user(conn: &mut SqliteConnection, new_user: NewUser) -> Option<User> {
    use crate::schema::users::dsl::*;

    match diesel::insert_into(users)
        .values(&new_user)
        .returning(User::as_returning())
        .get_result(conn)
    {
        Ok(user) => {
            info!("User '{}' created successfully", user.username);
            Some(user)
        }
        Err(e) => {
            error!("Error saving new user: {}", e);
            None
        }
    }
}

#[tauri::command]
pub fn create_wrestler(conn: &mut SqliteConnection, new_wrestler: NewWrestler) -> Option<Wrestler> {
    use crate::schema::wrestlers::dsl::*;

    match diesel::insert_into(wrestlers)
        .values(&new_wrestler)
        .returning(Wrestler::as_returning())
        .get_result(conn)
    {
        Ok(wrestler) => {
            info!("Wrestler '{}' created successfully", wrestler.name);
            Some(wrestler)
        }
        Err(e) => {
            error!("Error saving new wrestler: {}", e);
            None
        }
    }
}

#[tauri::command]
pub fn create_belt(conn: &mut SqliteConnection, new_title: NewTitle) -> Option<Title> {
    use crate::schema::titles::dsl::*;

    match diesel::insert_into(titles)
        .values(&new_title)
        .returning(Title::as_returning())
        .get_result(conn)
    {
        Ok(title) => {
            info!("Title '{}' created successfully", title.name);
            Some(title)
        }
        Err(e) => {
            error!("Error saving new title: {}", e);
            None
        }
    }
}
