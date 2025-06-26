use crate::models::{
    NewShow, NewTitle, NewUser, NewWrestler, Show, ShowData, Title, TitleData, User, UserData,
    Wrestler, WrestlerData,
};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use diesel::result::Error as DieselError;
use dotenvy::dotenv;
use log::{error, info};
use std::env;
use tauri::State;

pub type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;
pub type DbConnection = diesel::r2d2::PooledConnection<ConnectionManager<SqliteConnection>>;

pub struct DbState {
    pub pool: Pool,
}

/// Establishes a connection pool to the SQLite database
pub fn establish_connection() -> Pool {
    dotenv().expect("Error loading .env file");
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create database connection pool")
}

/// Gets a database connection from the pool
fn get_connection(state: &State<'_, DbState>) -> Result<DbConnection, String> {
    state.pool.get().map_err(|e| {
        error!("Failed to get database connection: {}", e);
        format!("Database connection error: {}", e)
    })
}

// ===== Show Operations =====

/// Creates a new show (used by tests and Tauri commands)
pub fn internal_create_show(
    conn: &mut SqliteConnection,
    name: &str,
    description: &str,
) -> Result<Show, DieselError> {
    let new_show = NewShow {
        name: name.to_string(),
        description: description.to_string(),
    };

    diesel::insert_into(crate::schema::shows::dsl::shows)
        .values(&new_show)
        .returning(Show::as_returning())
        .get_result(conn)
}

/// Gets all shows ordered by ID (used by tests and Tauri commands)
pub fn internal_get_shows(conn: &mut SqliteConnection) -> Result<Vec<Show>, DieselError> {
    use crate::schema::shows::dsl::*;
    shows.order(id.asc()).load::<Show>(conn)
}

#[tauri::command]
pub fn create_show(state: State<'_, DbState>, show_data: ShowData) -> Result<Show, String> {
    let mut conn = get_connection(&state)?;

    internal_create_show(&mut conn, &show_data.name, &show_data.description)
        .inspect(|show| {
            info!("Show '{}' created successfully", show.name);
        })
        .map_err(|e| {
            error!("Error creating show: {}", e);
            format!("Failed to create show: {}", e)
        })
}

#[tauri::command]
pub fn get_shows(state: State<'_, DbState>) -> Result<Vec<Show>, String> {
    let mut conn = get_connection(&state)?;

    internal_get_shows(&mut conn).map_err(|e| {
        error!("Error loading shows: {}", e);
        format!("Failed to load shows: {}", e)
    })
}

// ===== User Operations =====

/// Creates a new user (used by tests and Tauri commands)
pub fn internal_create_user(
    conn: &mut SqliteConnection,
    username: &str,
    password: &str,
) -> Result<User, DieselError> {
    let new_user = NewUser {
        username: username.to_string(),
        password: password.to_string(),
    };

    diesel::insert_into(crate::schema::users::dsl::users)
        .values(&new_user)
        .returning(User::as_returning())
        .get_result(conn)
}

#[tauri::command]
pub fn create_user(state: State<'_, DbState>, user_data: UserData) -> Result<User, String> {
    let mut conn = get_connection(&state)?;

    internal_create_user(&mut conn, &user_data.username, &user_data.password)
        .inspect(|user| {
            info!("User '{}' created successfully", user.username);
        })
        .map_err(|e| {
            error!("Error creating user: {}", e);
            format!("Failed to create user: {}", e)
        })
}

// ===== Wrestler Operations =====

/// Creates a new wrestler (used by tests and Tauri commands)
pub fn internal_create_wrestler(
    conn: &mut SqliteConnection,
    name: &str,
    gender: &str,
    wins: i32,
    losses: i32,
) -> Result<Wrestler, DieselError> {
    let new_wrestler = NewWrestler {
        name: name.to_string(),
        gender: gender.to_string(),
        wins,
        losses,
    };

    diesel::insert_into(crate::schema::wrestlers::dsl::wrestlers)
        .values(&new_wrestler)
        .returning(Wrestler::as_returning())
        .get_result(conn)
}

#[tauri::command]
pub fn create_wrestler(
    state: State<'_, DbState>,
    wrestler_data: WrestlerData,
) -> Result<Wrestler, String> {
    let mut conn = get_connection(&state)?;
    let gender_str: String = wrestler_data.gender.into();

    internal_create_wrestler(&mut conn, &wrestler_data.name, &gender_str, 0, 0)
        .inspect(|wrestler| {
            info!("Wrestler '{}' created successfully", wrestler.name);
        })
        .map_err(|e| {
            error!("Error creating wrestler: {}", e);
            format!("Failed to create wrestler: {}", e)
        })
}

// ===== Title Operations =====

/// Creates a new title/belt (used by tests and Tauri commands)
pub fn internal_create_belt(
    conn: &mut SqliteConnection,
    name: &str,
    current_holder_id: Option<i32>,
) -> Result<Title, DieselError> {
    let new_title = NewTitle {
        name: name.to_string(),
        current_holder_id,
    };

    diesel::insert_into(crate::schema::titles::dsl::titles)
        .values(&new_title)
        .returning(Title::as_returning())
        .get_result(conn)
}

#[tauri::command]
pub fn create_belt(state: State<'_, DbState>, title_data: TitleData) -> Result<Title, String> {
    let mut conn = get_connection(&state)?;

    internal_create_belt(&mut conn, &title_data.name, title_data.current_holder_id)
        .inspect(|title| {
            info!("Title '{}' created successfully", title.name);
        })
        .map_err(|e| {
            error!("Error creating title: {}", e);
            format!("Failed to create title: {}", e)
        })
}
