use crate::models::{NewTitle, NewUser, NewWrestler, Title, User, Wrestler, NewShow, Show, ShowData, UserData, WrestlerData, TitleData};
use diesel::prelude::*;
use dotenvy::dotenv;
use log::{error, info};
use std::env;
use tauri::State;
use std::sync::Mutex;
use diesel::result::Error as DieselError;

pub struct DbState {
    pub db: Mutex<SqliteConnection>,
}

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

// Internal function for database logic, testable directly
pub fn internal_create_show(conn: &mut SqliteConnection, name: &str, description: &str) -> Result<Show, DieselError> {
    // Create NewShow with owned Strings
    let new_show = NewShow {
        name: name.to_string(),
        description: description.to_string(),
    };

    // Explicitly qualify table name
    diesel::insert_into(crate::schema::shows::dsl::shows)
        .values(&new_show)
        .returning(Show::as_returning())
        .get_result(conn)
}

#[tauri::command]
pub fn create_show(state: State<'_, DbState>, show_data: ShowData) -> Result<Show, String> {
    let mut conn = state.db.lock().map_err(|e| e.to_string())?;

    // Call the internal function
    match internal_create_show(&mut *conn, &show_data.name, &show_data.description) {
        Ok(show) => {
            info!("Show '{}' created successfully", show.name);
            Ok(show)
        }
        Err(e) => {
            error!("Error saving new show: {}", e);
            Err(e.to_string()) // Convert DieselError to String for Tauri command return
        }
    }
}

pub fn internal_create_user(conn: &mut SqliteConnection, username: &str, password: &str) -> Result<User, DieselError> {
    let new_user = NewUser { username: username.to_string(), password: password.to_string() };

    // Explicitly qualify table name
    diesel::insert_into(crate::schema::users::dsl::users)
        .values(&new_user)
        .returning(User::as_returning())
        .get_result(conn)
}

#[tauri::command]
pub fn create_user(state: State<'_, DbState>, user_data: UserData) -> Result<User, String> {
    let mut conn = state.db.lock().map_err(|e| e.to_string())?;
    match internal_create_user(&mut *conn, &user_data.username, &user_data.password) {
        Ok(user) => {
            info!("User '{}' created successfully", user.username);
            Ok(user)
        }
        Err(e) => {
            error!("Error saving new user: {}", e);
            Err(e.to_string())
        }
    }
}

pub fn internal_create_wrestler(conn: &mut SqliteConnection, name: &str, gender: &str) -> Result<Wrestler, DieselError> {
    let new_wrestler = NewWrestler { name: name.to_string(), gender: gender.to_string() };

    // Explicitly qualify table name
    diesel::insert_into(crate::schema::wrestlers::dsl::wrestlers)
        .values(&new_wrestler)
        .returning(Wrestler::as_returning())
        .get_result(conn)
}

#[tauri::command]
pub fn create_wrestler(state: State<'_, DbState>, wrestler_data: WrestlerData) -> Result<Wrestler, String> {
    let mut conn = state.db.lock().map_err(|e| e.to_string())?;
    match internal_create_wrestler(&mut *conn, &wrestler_data.name, &wrestler_data.gender) {
        Ok(wrestler) => {
            info!("Wrestler '{}' created successfully", wrestler.name);
            Ok(wrestler)
        }
        Err(e) => {
            error!("Error saving new wrestler: {}", e);
            Err(e.to_string())
        }
    }
}

pub fn internal_create_belt(conn: &mut SqliteConnection, name: &str) -> Result<Title, DieselError> {
    let new_title = NewTitle { name: name.to_string() };

    // Explicitly qualify table name
    diesel::insert_into(crate::schema::titles::dsl::titles)
        .values(&new_title)
        .returning(Title::as_returning())
        .get_result(conn)
}

#[tauri::command]
pub fn create_belt(state: State<'_, DbState>, title_data: TitleData) -> Result<Title, String> {
    let mut conn = state.db.lock().map_err(|e| e.to_string())?;
    match internal_create_belt(&mut *conn, &title_data.name) {
        Ok(title) => {
            info!("Title '{}' created successfully", title.name);
            Ok(title)
        }
        Err(e) => {
            error!("Error saving new title: {}", e);
            Err(e.to_string())
        }
    }
}

pub fn internal_get_shows(conn: &mut SqliteConnection) -> Result<Vec<Show>, DieselError> {
    // Explicitly qualify table name
    crate::schema::shows::dsl::shows.load::<Show>(conn)
}

#[tauri::command]
pub fn get_shows(state: State<'_, DbState>) -> Result<Vec<Show>, String> {
    let mut conn = state.db.lock().map_err(|e| e.to_string())?;
    match internal_get_shows(&mut *conn) {
        Ok(all_shows) => Ok(all_shows),
        Err(e) => Err(format!("Error loading shows: {}", e)),
    }
}
