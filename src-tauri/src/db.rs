use crate::models::{
    Match, MatchData, NewMatch, MatchParticipant, NewMatchParticipant,
    NewShowRoster, NewShow, NewSignatureMove, NewTitle, NewTitleHolder, NewUser, NewWrestler, NewEnhancedWrestler, ShowRoster, Show, ShowData, SignatureMove, Title, TitleData, TitleHolder, TitleWithHolders, TitleHolderInfo, User, UserData,
    Wrestler, WrestlerData, EnhancedWrestlerData,
};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use diesel::result::Error as DieselError;
use dotenvy::dotenv;
use log::{error, info};
use std::env;
use tauri::State;
use chrono::Utc;

/// Type alias for the database connection pool
pub type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

/// Type alias for a pooled database connection
pub type DbConnection = diesel::r2d2::PooledConnection<ConnectionManager<SqliteConnection>>;

/// State struct that holds the database connection pool for Tauri state management
pub struct DbState {
    /// The r2d2 connection pool for SQLite
    pub pool: Pool,
}

/// Establishes a connection pool to the SQLite database
/// 
/// # Returns
/// A configured r2d2 connection pool for SQLite
/// 
/// # Panics
/// - If the .env file cannot be loaded
/// - If DATABASE_URL environment variable is not set
/// - If the connection pool cannot be created
pub fn establish_connection() -> Pool {
    dotenv().expect("Error loading .env file");
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create database connection pool")
}

/// Gets a database connection from the pool
/// 
/// # Arguments
/// * `state` - The Tauri state containing the database connection pool
/// 
/// # Returns
/// * `Ok(DbConnection)` - A pooled database connection
/// * `Err(String)` - Error message if connection cannot be obtained
fn get_connection(state: &State<'_, DbState>) -> Result<DbConnection, String> {
    state.pool.get().map_err(|e| {
        error!("Failed to get database connection: {}", e);
        format!("Database connection error: {}", e)
    })
}

// ===== Show Operations =====

/// Creates a new show in the database (internal function for tests and commands)
/// 
/// # Arguments
/// * `conn` - Mutable reference to the database connection
/// * `name` - The name of the show (e.g., "Monday Night RAW")
/// * `description` - Description of the show
/// 
/// # Returns
/// * `Ok(Show)` - The newly created show with generated ID
/// * `Err(DieselError)` - Database error if creation fails
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

/// Gets all shows ordered by ID (internal function for tests and commands)
/// 
/// # Arguments
/// * `conn` - Mutable reference to the database connection
/// 
/// # Returns
/// * `Ok(Vec<Show>)` - Vector of all shows ordered by ID ascending
/// * `Err(DieselError)` - Database error if query fails
pub fn internal_get_shows(conn: &mut SqliteConnection) -> Result<Vec<Show>, DieselError> {
    use crate::schema::shows::dsl::*;
    shows
        .order(id.asc())
        .load::<Show>(conn)
}

/// Tauri command to create a new wrestling show
/// 
/// # Arguments
/// * `state` - The Tauri state containing the database pool
/// * `show_data` - ShowData struct containing name and description
/// 
/// # Returns
/// * `Ok(Show)` - The newly created show
/// * `Err(String)` - Error message if creation fails
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

/// Tauri command to fetch all wrestling shows
/// 
/// # Arguments
/// * `state` - The Tauri state containing the database pool
/// 
/// # Returns
/// * `Ok(Vec<Show>)` - Vector of all shows in the database
/// * `Err(String)` - Error message if query fails
#[tauri::command]
pub fn get_shows(state: State<'_, DbState>) -> Result<Vec<Show>, String> {
    let mut conn = get_connection(&state)?;

    internal_get_shows(&mut conn).map_err(|e| {
        error!("Error loading shows: {}", e);
        format!("Failed to load shows: {}", e)
    })
}


// ===== User Operations =====

/// Creates a new user in the database (internal function for tests and commands)
/// 
/// # Arguments
/// * `conn` - Mutable reference to the database connection
/// * `username` - The username for the new user
/// * `password` - The password (will be hashed before storage)
/// 
/// # Returns
/// * `Ok(User)` - The newly created user
/// * `Err(DieselError)` - Database error if creation fails
/// 
/// # Note
/// Passwords should be hashed before calling this function
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

/// Tauri command to create a new user account
/// 
/// # Arguments
/// * `state` - The Tauri state containing the database pool
/// * `user_data` - UserData struct containing username and password
/// 
/// # Returns
/// * `Ok(User)` - The newly created user
/// * `Err(String)` - Error message if creation fails
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

/// Gets all wrestlers ordered by ID (internal function for tests and commands)
/// 
/// # Arguments
/// * `conn` - Mutable reference to the database connection
/// 
/// # Returns
/// * `Ok(Vec<Wrestler>)` - Vector of all wrestlers in the global pool
/// * `Err(DieselError)` - Database error if query fails
/// 
/// # Note
/// Wrestlers are global entities not tied to specific promotions
pub fn internal_get_wrestlers(conn: &mut SqliteConnection) -> Result<Vec<Wrestler>, DieselError> {
    use crate::schema::wrestlers::dsl::*;
    wrestlers
        .order(id.asc())
        .load::<Wrestler>(conn)
}

/// Gets a specific wrestler by ID (internal function for tests and commands)
/// 
/// # Arguments
/// * `conn` - Mutable reference to the database connection
/// * `wrestler_id` - The ID of the wrestler to retrieve
/// 
/// # Returns
/// * `Ok(Some(Wrestler))` - The wrestler if found
/// * `Ok(None)` - If no wrestler with the given ID exists
/// * `Err(DieselError)` - Database error if query fails
pub fn internal_get_wrestler_by_id(conn: &mut SqliteConnection, wrestler_id: i32) -> Result<Option<Wrestler>, DieselError> {
    use crate::schema::wrestlers::dsl::*;
    wrestlers.filter(id.eq(wrestler_id)).first::<Wrestler>(conn).optional()
}

/// Creates a new wrestler with basic information (internal function)
/// 
/// # Arguments
/// * `conn` - Mutable reference to the database connection
/// * `name` - The wrestler's ring name
/// * `gender` - Gender ("Male", "Female", or other)
/// * `wins` - Number of wins
/// * `losses` - Number of losses
/// 
/// # Returns
/// * `Ok(Wrestler)` - The newly created wrestler
/// * `Err(DieselError)` - Database error if creation fails
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
        is_user_created: Some(false), // Default to system wrestler
    };

    diesel::insert_into(crate::schema::wrestlers::dsl::wrestlers)
        .values(&new_wrestler)
        .returning(Wrestler::as_returning())
        .get_result(conn)
}

/// Creates a new wrestler with enhanced details (internal function for test data)
/// 
/// # Arguments
/// * `conn` - Mutable reference to the database connection
/// * `wrestler_name` - Ring name
/// * `wrestler_real_name` - Real name
/// * `wrestler_nickname` - Nickname or catchphrase
/// * `wrestler_gender` - Gender ("Male", "Female", or other)
/// * `wrestler_wins` - Initial win count
/// * `wrestler_losses` - Initial loss count
/// * `wrestler_height` - Height (e.g., "6'5\"")
/// * `wrestler_weight` - Weight (e.g., "260 lbs")
/// * `wrestler_debut_year` - Year of wrestling debut
/// * `wrestler_strength` - Strength rating (1-10)
/// * `wrestler_speed` - Speed rating (1-10)
/// * `wrestler_agility` - Agility rating (1-10)
/// * `wrestler_stamina` - Stamina rating (1-10)
/// * `wrestler_charisma` - Charisma rating (1-10)
/// * `wrestler_technique` - Technique rating (1-10)
/// * `wrestler_biography` - Biography text
/// * `is_user_created` - Whether this is a user-created wrestler
/// 
/// # Returns
/// * `Ok(Wrestler)` - The newly created wrestler with all details
/// * `Err(DieselError)` - Database error if creation fails
pub fn internal_create_enhanced_wrestler(
    conn: &mut SqliteConnection,
    wrestler_name: &str,
    wrestler_real_name: &str,
    wrestler_nickname: &str,
    wrestler_gender: &str,
    wrestler_wins: i32,
    wrestler_losses: i32,
    wrestler_height: &str,
    wrestler_weight: &str,
    wrestler_debut_year: i32,
    wrestler_strength: i32,
    wrestler_speed: i32,
    wrestler_agility: i32,
    wrestler_stamina: i32,
    wrestler_charisma: i32,
    wrestler_technique: i32,
    wrestler_biography: &str,
    is_user_created: bool,
) -> Result<Wrestler, DieselError> {
    let new_wrestler = NewEnhancedWrestler {
        name: wrestler_name.to_string(),
        gender: wrestler_gender.to_string(),
        wins: wrestler_wins,
        losses: wrestler_losses,
        real_name: Some(wrestler_real_name.to_string()),
        nickname: Some(wrestler_nickname.to_string()),
        height: Some(wrestler_height.to_string()),
        weight: Some(wrestler_weight.to_string()),
        debut_year: Some(wrestler_debut_year),
        strength: Some(wrestler_strength),
        speed: Some(wrestler_speed),
        agility: Some(wrestler_agility),
        stamina: Some(wrestler_stamina),
        charisma: Some(wrestler_charisma),
        technique: Some(wrestler_technique),
        biography: Some(wrestler_biography.to_string()),
        is_user_created: Some(is_user_created),
    };

    diesel::insert_into(crate::schema::wrestlers::dsl::wrestlers)
        .values(&new_wrestler)
        .returning(Wrestler::as_returning())
        .get_result(conn)
}

/// Creates a new user-created wrestler with enhanced details
/// 
/// # Arguments
/// * `conn` - Mutable reference to the database connection
/// * `wrestler_data` - EnhancedWrestlerData struct with all wrestler details
/// 
/// # Returns
/// * `Ok(Wrestler)` - The newly created wrestler marked as user-created
/// * `Err(DieselError)` - Database error if creation fails
/// 
/// # Note
/// New wrestlers start with 0 wins and losses
pub fn internal_create_user_wrestler(
    conn: &mut SqliteConnection,
    wrestler_data: &EnhancedWrestlerData,
) -> Result<Wrestler, DieselError> {
    let gender_str: String = wrestler_data.gender.clone().into();
    
    let new_wrestler = NewEnhancedWrestler {
        name: wrestler_data.name.clone(),
        gender: gender_str,
        wins: 0, // New wrestlers start with 0 wins/losses
        losses: 0,
        real_name: wrestler_data.real_name.clone(),
        nickname: wrestler_data.nickname.clone(),
        height: wrestler_data.height.clone(),
        weight: wrestler_data.weight.clone(),
        debut_year: wrestler_data.debut_year,
        strength: wrestler_data.strength,
        speed: wrestler_data.speed,
        agility: wrestler_data.agility,
        stamina: wrestler_data.stamina,
        charisma: wrestler_data.charisma,
        technique: wrestler_data.technique,
        biography: wrestler_data.biography.clone(),
        is_user_created: Some(true), // User-created wrestler
        // Wrestlers are now global - no promotion_id needed
    };

    diesel::insert_into(crate::schema::wrestlers::dsl::wrestlers)
        .values(&new_wrestler)
        .returning(Wrestler::as_returning())
        .get_result(conn)
}


/// Updates a wrestler's power ratings
/// 
/// # Arguments
/// * `conn` - Mutable reference to the database connection
/// * `wrestler_id` - ID of the wrestler to update
/// * `new_strength` - New strength rating (1-10, None to keep existing)
/// * `new_speed` - New speed rating (1-10, None to keep existing)
/// * `new_agility` - New agility rating (1-10, None to keep existing)
/// * `new_stamina` - New stamina rating (1-10, None to keep existing)
/// * `new_charisma` - New charisma rating (1-10, None to keep existing)
/// * `new_technique` - New technique rating (1-10, None to keep existing)
/// 
/// # Returns
/// * `Ok(Wrestler)` - The updated wrestler
/// * `Err(DieselError)` - Database error if update fails
pub fn internal_update_wrestler_power_ratings(
    conn: &mut SqliteConnection,
    wrestler_id: i32,
    new_strength: Option<i32>,
    new_speed: Option<i32>,
    new_agility: Option<i32>,
    new_stamina: Option<i32>,
    new_charisma: Option<i32>,
    new_technique: Option<i32>,
) -> Result<Wrestler, DieselError> {
    use crate::schema::wrestlers::dsl::*;
    
    diesel::update(wrestlers.filter(id.eq(wrestler_id)))
        .set((
            strength.eq(new_strength),
            speed.eq(new_speed),
            agility.eq(new_agility),
            stamina.eq(new_stamina),
            charisma.eq(new_charisma),
            technique.eq(new_technique),
        ))
        .returning(Wrestler::as_returning())
        .get_result(conn)
}

/// Updates a wrestler's basic statistics and physical attributes
/// 
/// # Arguments
/// * `conn` - Mutable reference to the database connection
/// * `wrestler_id` - ID of the wrestler to update
/// * `new_height` - New height (e.g., "6'2\"", None to keep existing)
/// * `new_weight` - New weight (e.g., "220 lbs", None to keep existing)
/// * `new_debut_year` - New debut year (None to keep existing)
/// * `new_wins` - New win count
/// * `new_losses` - New loss count
/// 
/// # Returns
/// * `Ok(Wrestler)` - The updated wrestler
/// * `Err(DieselError)` - Database error if update fails
pub fn internal_update_wrestler_basic_stats(
    conn: &mut SqliteConnection,
    wrestler_id: i32,
    new_height: Option<String>,
    new_weight: Option<String>,
    new_debut_year: Option<i32>,
    new_wins: i32,
    new_losses: i32,
) -> Result<Wrestler, DieselError> {
    use crate::schema::wrestlers::dsl::*;
    
    diesel::update(wrestlers.filter(id.eq(wrestler_id)))
        .set((
            height.eq(new_height),
            weight.eq(new_weight),
            debut_year.eq(new_debut_year),
            wins.eq(new_wins),
            losses.eq(new_losses),
        ))
        .returning(Wrestler::as_returning())
        .get_result(conn)
}

/// Updates a wrestler's ring name and nickname
/// 
/// # Arguments
/// * `conn` - Mutable reference to the database connection
/// * `wrestler_id` - ID of the wrestler to update
/// * `new_name` - New ring name
/// * `new_nickname` - New nickname or catchphrase (None to clear)
/// 
/// # Returns
/// * `Ok(Wrestler)` - The updated wrestler
/// * `Err(DieselError)` - Database error if update fails
pub fn internal_update_wrestler_name(
    conn: &mut SqliteConnection,
    wrestler_id: i32,
    new_name: &str,
    new_nickname: Option<String>,
) -> Result<Wrestler, DieselError> {
    use crate::schema::wrestlers::dsl::*;
    
    diesel::update(wrestlers.filter(id.eq(wrestler_id)))
        .set((
            name.eq(new_name),
            nickname.eq(new_nickname),
        ))
        .returning(Wrestler::as_returning())
        .get_result(conn)
}

/// Updates a wrestler's real name
/// 
/// # Arguments
/// * `conn` - Mutable reference to the database connection
/// * `wrestler_id` - ID of the wrestler to update
/// * `new_real_name` - New real name (None to clear)
/// 
/// # Returns
/// * `Ok(Wrestler)` - The updated wrestler
/// * `Err(DieselError)` - Database error if update fails
pub fn internal_update_wrestler_real_name(
    conn: &mut SqliteConnection,
    wrestler_id: i32,
    new_real_name: Option<String>,
) -> Result<Wrestler, DieselError> {
    use crate::schema::wrestlers::dsl::*;
    
    diesel::update(wrestlers.filter(id.eq(wrestler_id)))
        .set(real_name.eq(new_real_name))
        .returning(Wrestler::as_returning())
        .get_result(conn)
}

/// Updates a wrestler's biography text
/// 
/// # Arguments
/// * `conn` - Mutable reference to the database connection
/// * `wrestler_id` - ID of the wrestler to update
/// * `new_biography` - New biography text (None to clear)
/// 
/// # Returns
/// * `Ok(Wrestler)` - The updated wrestler
/// * `Err(DieselError)` - Database error if update fails
pub fn internal_update_wrestler_biography(
    conn: &mut SqliteConnection,
    wrestler_id: i32,
    new_biography: Option<String>,
) -> Result<Wrestler, DieselError> {
    use crate::schema::wrestlers::dsl::*;
    
    diesel::update(wrestlers.filter(id.eq(wrestler_id)))
        .set(biography.eq(new_biography))
        .returning(Wrestler::as_returning())
        .get_result(conn)
}


/// Creates a new signature move for a wrestler
/// 
/// # Arguments
/// * `conn` - Mutable reference to the database connection
/// * `wrestler_id` - ID of the wrestler who performs this move
/// * `move_name` - Name of the move (e.g., "Stone Cold Stunner")
/// * `move_type` - Type of move ("primary", "secondary", etc.)
/// 
/// # Returns
/// * `Ok(SignatureMove)` - The newly created signature move
/// * `Err(DieselError)` - Database error if creation fails
pub fn internal_create_signature_move(
    conn: &mut SqliteConnection,
    wrestler_id: i32,
    move_name: &str,
    move_type: &str,
) -> Result<SignatureMove, DieselError> {
    let new_move = NewSignatureMove {
        wrestler_id,
        move_name: move_name.to_string(),
        move_type: move_type.to_string(),
    };

    diesel::insert_into(crate::schema::signature_moves::dsl::signature_moves)
        .values(&new_move)
        .returning(SignatureMove::as_returning())
        .get_result(conn)
}

/// Deletes a wrestler (only if user-created)
/// 
/// # Arguments
/// * `conn` - Mutable reference to the database connection
/// * `wrestler_id` - ID of the wrestler to delete
/// 
/// # Returns
/// * `Ok(())` - If deletion was successful
/// * `Err(DieselError::RollbackTransaction)` - If wrestler is not user-created
/// * `Err(DieselError)` - Other database errors
/// 
/// # Note
/// System wrestlers cannot be deleted. Deletion cascades through foreign keys.
pub fn internal_delete_wrestler(
    conn: &mut SqliteConnection,
    wrestler_id: i32,
) -> Result<(), DieselError> {
    use crate::schema::wrestlers::dsl::*;
    
    // First check if the wrestler exists and is user-created
    let wrestler = wrestlers
        .filter(id.eq(wrestler_id))
        .first::<Wrestler>(conn)?;
    
    // Only allow deletion of user-created wrestlers
    if !wrestler.is_user_created.unwrap_or(false) {
        return Err(DieselError::RollbackTransaction);
    }
    
    // Delete the wrestler (database handles cascading deletes via foreign key constraints)
    diesel::delete(wrestlers.filter(id.eq(wrestler_id)))
        .execute(conn)?;
        
    Ok(())
}

/// Tauri command to create a new wrestler with basic information
/// 
/// # Arguments
/// * `state` - The Tauri state containing the database pool
/// * `wrestler_data` - WrestlerData struct containing name and gender
/// 
/// # Returns
/// * `Ok(Wrestler)` - The newly created wrestler
/// * `Err(String)` - Error message if creation fails
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

/// Tauri command to create a user-defined wrestler with full details
/// 
/// # Arguments
/// * `state` - The Tauri state containing the database pool
/// * `wrestler_data` - EnhancedWrestlerData with all wrestler attributes
/// 
/// # Returns
/// * `Ok(Wrestler)` - The newly created wrestler
/// * `Err(String)` - Error message if creation fails
#[tauri::command]
pub fn create_user_wrestler(
    state: State<'_, DbState>,
    wrestler_data: EnhancedWrestlerData,
) -> Result<Wrestler, String> {
    let mut conn = get_connection(&state)?;

    internal_create_user_wrestler(&mut conn, &wrestler_data)
        .inspect(|wrestler| {
            info!("User wrestler '{}' created successfully", wrestler.name);
        })
        .map_err(|e| {
            error!("Error creating user wrestler: {}", e);
            format!("Failed to create user wrestler: {}", e)
        })
}

/// Tauri command to fetch all wrestlers from the global pool
/// 
/// # Arguments
/// * `state` - The Tauri state containing the database pool
/// 
/// # Returns
/// * `Ok(Vec<Wrestler>)` - Vector of all wrestlers
/// * `Err(String)` - Error message if query fails
#[tauri::command]
pub fn get_wrestlers(state: State<'_, DbState>) -> Result<Vec<Wrestler>, String> {
    let mut conn = get_connection(&state)?;

    internal_get_wrestlers(&mut conn).map_err(|e| {
        error!("Error loading wrestlers: {}", e);
        format!("Failed to load wrestlers: {}", e)
    })
}

/// Tauri command to fetch all unassigned wrestlers (not on any show roster)
/// 
/// # Arguments
/// * `state` - The Tauri state containing the database pool
/// 
/// # Returns
/// * `Ok(Vec<Wrestler>)` - Vector of wrestlers not assigned to any show
/// * `Err(String)` - Error message if query fails
#[tauri::command]
pub fn get_unassigned_wrestlers(state: State<'_, DbState>) -> Result<Vec<Wrestler>, String> {
    let mut conn = get_connection(&state)?;

    internal_get_unassigned_wrestlers(&mut conn).map_err(|e| {
        error!("Error loading unassigned wrestlers: {}", e);
        format!("Failed to load unassigned wrestlers: {}", e)
    })
}

/// Tauri command to fetch a specific wrestler by ID
/// 
/// # Arguments
/// * `state` - The Tauri state containing the database pool
/// * `wrestler_id` - The ID of the wrestler to retrieve
/// 
/// # Returns
/// * `Ok(Some(Wrestler))` - The wrestler if found
/// * `Ok(None)` - If no wrestler with the given ID exists
/// * `Err(String)` - Error message if query fails
#[tauri::command]
pub fn get_wrestler_by_id(state: State<'_, DbState>, wrestler_id: i32) -> Result<Option<Wrestler>, String> {
    let mut conn = get_connection(&state)?;

    internal_get_wrestler_by_id(&mut conn, wrestler_id).map_err(|e| {
        error!("Error loading wrestler: {}", e);
        format!("Failed to load wrestler: {}", e)
    })
}


/// Tauri command to update a wrestler's power ratings
/// 
/// # Arguments
/// * `state` - The Tauri state containing the database pool
/// * `wrestler_id` - ID of the wrestler to update
/// * `strength` - New strength rating (1-10, None to keep)
/// * `speed` - New speed rating (1-10, None to keep)
/// * `agility` - New agility rating (1-10, None to keep)
/// * `stamina` - New stamina rating (1-10, None to keep)
/// * `charisma` - New charisma rating (1-10, None to keep)
/// * `technique` - New technique rating (1-10, None to keep)
/// 
/// # Returns
/// * `Ok(Wrestler)` - The updated wrestler
/// * `Err(String)` - Error message if update fails
#[tauri::command]
pub fn update_wrestler_power_ratings(
    state: State<'_, DbState>,
    wrestler_id: i32,
    strength: Option<i32>,
    speed: Option<i32>,
    agility: Option<i32>,
    stamina: Option<i32>,
    charisma: Option<i32>,
    technique: Option<i32>,
) -> Result<Wrestler, String> {
    let mut conn = get_connection(&state)?;

    internal_update_wrestler_power_ratings(
        &mut conn, 
        wrestler_id, 
        strength, 
        speed, 
        agility, 
        stamina, 
        charisma, 
        technique
    )
    .inspect(|wrestler| {
        info!("Wrestler '{}' power ratings updated", wrestler.name);
    })
    .map_err(|e| {
        error!("Error updating wrestler power ratings: {}", e);
        format!("Failed to update wrestler power ratings: {}", e)
    })
}

/// Tauri command to update a wrestler's basic statistics
/// 
/// # Arguments
/// * `state` - The Tauri state containing the database pool
/// * `wrestler_id` - ID of the wrestler to update
/// * `height` - New height (e.g., "6'2\"")
/// * `weight` - New weight (e.g., "220 lbs")
/// * `debut_year` - New debut year
/// * `wins` - New win count
/// * `losses` - New loss count
/// 
/// # Returns
/// * `Ok(Wrestler)` - The updated wrestler
/// * `Err(String)` - Error message if update fails
#[tauri::command]
pub fn update_wrestler_basic_stats(
    state: State<'_, DbState>,
    wrestler_id: i32,
    height: Option<String>,
    weight: Option<String>,
    debut_year: Option<i32>,
    wins: i32,
    losses: i32,
) -> Result<Wrestler, String> {
    let mut conn = get_connection(&state)?;

    internal_update_wrestler_basic_stats(
        &mut conn, 
        wrestler_id, 
        height, 
        weight, 
        debut_year, 
        wins, 
        losses
    )
    .inspect(|wrestler| {
        info!("Wrestler '{}' basic stats updated", wrestler.name);
    })
    .map_err(|e| {
        error!("Error updating wrestler basic stats: {}", e);
        format!("Failed to update wrestler basic stats: {}", e)
    })
}

/// Tauri command to update a wrestler's ring name and nickname
/// 
/// # Arguments
/// * `state` - The Tauri state containing the database pool
/// * `wrestler_id` - ID of the wrestler to update
/// * `name` - New ring name
/// * `nickname` - New nickname or catchphrase
/// 
/// # Returns
/// * `Ok(Wrestler)` - The updated wrestler
/// * `Err(String)` - Error message if update fails
#[tauri::command]
pub fn update_wrestler_name(
    state: State<'_, DbState>,
    wrestler_id: i32,
    name: String,
    nickname: Option<String>,
) -> Result<Wrestler, String> {
    let mut conn = get_connection(&state)?;

    internal_update_wrestler_name(&mut conn, wrestler_id, &name, nickname)
        .inspect(|wrestler| {
            info!("Wrestler '{}' name updated", wrestler.name);
        })
        .map_err(|e| {
            error!("Error updating wrestler name: {}", e);
            format!("Failed to update wrestler name: {}", e)
        })
}

/// Tauri command to update a wrestler's real name
/// 
/// # Arguments
/// * `state` - The Tauri state containing the database pool
/// * `wrestler_id` - ID of the wrestler to update
/// * `real_name` - New real name (None to clear)
/// 
/// # Returns
/// * `Ok(Wrestler)` - The updated wrestler
/// * `Err(String)` - Error message if update fails
#[tauri::command]
pub fn update_wrestler_real_name(
    state: State<'_, DbState>,
    wrestler_id: i32,
    real_name: Option<String>,
) -> Result<Wrestler, String> {
    let mut conn = get_connection(&state)?;

    internal_update_wrestler_real_name(&mut conn, wrestler_id, real_name)
        .inspect(|wrestler| {
            info!("Wrestler '{}' real name updated", wrestler.name);
        })
        .map_err(|e| {
            error!("Error updating wrestler real name: {}", e);
            format!("Failed to update wrestler real name: {}", e)
        })
}

/// Tauri command to update a wrestler's biography
/// 
/// # Arguments
/// * `state` - The Tauri state containing the database pool
/// * `wrestler_id` - ID of the wrestler to update
/// * `biography` - New biography text (None to clear)
/// 
/// # Returns
/// * `Ok(Wrestler)` - The updated wrestler
/// * `Err(String)` - Error message if update fails
#[tauri::command]
pub fn update_wrestler_biography(
    state: State<'_, DbState>,
    wrestler_id: i32,
    biography: Option<String>,
) -> Result<Wrestler, String> {
    let mut conn = get_connection(&state)?;

    internal_update_wrestler_biography(&mut conn, wrestler_id, biography)
        .inspect(|wrestler| {
            info!("Wrestler '{}' biography updated", wrestler.name);
        })
        .map_err(|e| {
            error!("Error updating wrestler biography: {}", e);
            format!("Failed to update wrestler biography: {}", e)
        })
}

/// Tauri command to delete a wrestler (only user-created wrestlers)
/// 
/// # Arguments
/// * `state` - The Tauri state containing the database pool
/// * `wrestler_id` - ID of the wrestler to delete
/// 
/// # Returns
/// * `Ok(String)` - Success message
/// * `Err(String)` - Error message if deletion fails or wrestler is system-created
#[tauri::command]
pub fn delete_wrestler(state: State<'_, DbState>, wrestler_id: i32) -> Result<String, String> {
    let mut conn = get_connection(&state)?;

    internal_delete_wrestler(&mut conn, wrestler_id)
        .inspect(|_| {
            info!("Wrestler with ID {} deleted successfully", wrestler_id);
        })
        .map_err(|e| {
            error!("Error deleting wrestler: {}", e);
            match e {
                DieselError::RollbackTransaction => "Cannot delete system wrestler - only user-created wrestlers can be deleted".to_string(),
                DieselError::NotFound => "Wrestler not found".to_string(),
                _ => format!("Failed to delete wrestler: {}", e),
            }
        })
        .map(|_| "Wrestler deleted successfully".to_string())
}


// ===== Title Operations =====

/// Creates a new championship title (internal function)
/// 
/// # Arguments
/// * `conn` - Mutable reference to the database connection
/// * `name` - Title name (e.g., "WWE Championship")
/// * `title_type` - Type of title ("Singles", "Tag Team", etc.)
/// * `division` - Division ("World", "Intercontinental", etc.)
/// * `gender` - Gender restriction ("Male", "Female", "Mixed")
/// * `show_id` - Optional show assignment (None for cross-brand)
/// * `current_holder_id` - Optional initial champion ID
/// * `is_user_created` - Whether this is a user-created title
/// 
/// # Returns
/// * `Ok(Title)` - The newly created title
/// * `Err(DieselError)` - Database error if creation fails
/// 
/// # Note
/// Prestige tier is automatically calculated based on division
pub fn internal_create_belt(
    conn: &mut SqliteConnection,
    name: &str,
    title_type: &str,
    division: &str,
    gender: &str,
    show_id: Option<i32>,
    current_holder_id: Option<i32>,
    is_user_created: bool,
) -> Result<Title, DieselError> {
    // Calculate prestige tier based on division
    let prestige_tier = match division {
        "World" | "WWE Championship" | "Women's World" | "WWE Women's Championship" => 1,
        "Intercontinental" | "United States" | "Women's Intercontinental" | "Women's United States" => 2,
        "World Tag Team" | "WWE Tag Team" | "Women's Tag Team" => 3,
        _ => 4, // Specialty titles
    };

    let new_title = NewTitle {
        name: name.to_string(),
        current_holder_id,
        title_type: title_type.to_string(),
        division: division.to_string(),
        prestige_tier,
        gender: gender.to_string(),
        show_id,
        is_active: true,
        is_user_created: Some(is_user_created),
    };

    diesel::insert_into(crate::schema::titles::dsl::titles)
        .values(&new_title)
        .returning(Title::as_returning())
        .get_result(conn)
}

/// Tauri command to create a new championship title
/// 
/// # Arguments
/// * `state` - The Tauri state containing the database pool
/// * `title_data` - TitleData struct with all title attributes
/// 
/// # Returns
/// * `Ok(Title)` - The newly created title
/// * `Err(String)` - Error message if creation fails
#[tauri::command]
pub fn create_belt(state: State<'_, DbState>, title_data: TitleData) -> Result<Title, String> {
    let mut conn = get_connection(&state)?;

    internal_create_belt(
        &mut conn,
        &title_data.name,
        &title_data.title_type,
        &title_data.division,
        &title_data.gender,
        title_data.show_id,
        title_data.current_holder_id,
        true, // User-created titles
    )
    .inspect(|title| {
        info!("Title '{}' created successfully", title.name);
    })
    .map_err(|e| {
        error!("Error creating title: {}", e);
        format!("Failed to create title: {}", e)
    })
}

/// Gets all titles with their current holders (internal function)
/// 
/// # Arguments
/// * `conn` - Mutable reference to the database connection
/// 
/// # Returns
/// * `Ok(Vec<TitleWithHolders>)` - Vector of titles with holder information
/// * `Err(DieselError)` - Database error if query fails
/// 
/// # Note
/// Returns all active titles from the global pool, ordered by prestige tier
pub fn internal_get_titles(conn: &mut SqliteConnection) -> Result<Vec<TitleWithHolders>, DieselError> {
    use crate::schema::{titles, title_holders, wrestlers};
    
    // Get all active titles (global, not promotion-specific)
    let all_titles = titles::table
        .filter(titles::is_active.eq(true))
        .order(titles::prestige_tier.asc())
        .then_order_by(titles::name.asc())
        .load::<Title>(conn)?;

    let mut titles_with_holders = Vec::new();

    for title in all_titles {
        // Get current holders for this title
        let current_holders_data = title_holders::table
            .inner_join(wrestlers::table.on(title_holders::wrestler_id.eq(wrestlers::id)))
            .filter(title_holders::title_id.eq(title.id))
            .filter(title_holders::held_until.is_null())
            .select((TitleHolder::as_select(), wrestlers::name, wrestlers::gender))
            .load::<(TitleHolder, String, String)>(conn)?;

        let current_holders: Vec<TitleHolderInfo> = current_holders_data
            .into_iter()
            .map(|(holder, wrestler_name, wrestler_gender)| TitleHolderInfo {
                holder,
                wrestler_name,
                wrestler_gender,
            })
            .collect();

        // Calculate days held for the first holder (for single titles)
        let days_held = if let Some(first_holder) = current_holders.first() {
            let now = Utc::now().naive_utc();
            let duration = now - first_holder.holder.held_since;
            Some(duration.num_days() as i32)
        } else {
            None
        };

        titles_with_holders.push(TitleWithHolders {
            title,
            current_holders,
            days_held,
        });
    }

    Ok(titles_with_holders)
}

/// Tauri command to fetch all championship titles with holders
/// 
/// # Arguments
/// * `state` - The Tauri state containing the database pool
/// 
/// # Returns
/// * `Ok(Vec<TitleWithHolders>)` - Vector of all titles with current holders
/// * `Err(String)` - Error message if query fails
#[tauri::command]
pub fn get_titles(state: State<'_, DbState>) -> Result<Vec<TitleWithHolders>, String> {
    let mut conn = get_connection(&state)?;
    
    internal_get_titles(&mut conn)
        .map_err(|e| {
            error!("Error fetching titles: {}", e);
            format!("Failed to fetch titles: {}", e)
        })
}

/// Deletes a championship title (only if user-created)
/// 
/// # Arguments
/// * `conn` - Mutable reference to the database connection
/// * `title_id` - ID of the title to delete
/// 
/// # Returns
/// * `Ok(())` - If deletion was successful
/// * `Err(DieselError::RollbackTransaction)` - If title is not user-created
/// * `Err(DieselError)` - Other database errors
/// 
/// # Note
/// Ends any current title reigns before deletion
pub fn internal_delete_title(
    conn: &mut SqliteConnection,
    title_id: i32,
) -> Result<(), DieselError> {
    use crate::schema::{titles, title_holders};
    
    // First check if the title exists and is user-created
    let title = titles::table
        .filter(titles::id.eq(title_id))
        .first::<Title>(conn)?;
    
    // Only allow deletion of user-created titles
    if !title.is_user_created.unwrap_or(false) {
        return Err(DieselError::RollbackTransaction);
    }
    
    // End any current title reigns for this title before deletion
    let now = Utc::now().naive_utc();
    diesel::update(title_holders::table)
        .filter(title_holders::title_id.eq(title_id))
        .filter(title_holders::held_until.is_null())
        .set(title_holders::held_until.eq(now))
        .execute(conn)?;
    
    // Delete the title (database handles cascading deletes via foreign key constraints)
    diesel::delete(titles::table.filter(titles::id.eq(title_id)))
        .execute(conn)?;
        
    Ok(())
}

/// Gets titles that can be assigned to a wrestler based on gender compatibility
/// 
/// # Arguments
/// * `conn` - Mutable reference to the database connection
/// * `wrestler_gender` - Gender of the wrestler ("Male", "Female", etc.)
/// 
/// # Returns
/// * `Ok(Vec<TitleWithHolders>)` - Vector of compatible titles
/// * `Err(DieselError)` - Database error if query fails
/// 
/// # Note
/// - Male wrestlers can hold Male and Mixed titles
/// - Female wrestlers can hold Female and Mixed titles
/// - Other gender wrestlers can hold any title
pub fn internal_get_titles_for_wrestler_gender(
    conn: &mut SqliteConnection,
    wrestler_gender: &str,
) -> Result<Vec<TitleWithHolders>, DieselError> {
    use crate::schema::{titles, title_holders, wrestlers};
    
    // Filter titles based on gender compatibility:
    // - Male wrestlers can hold Male and Mixed titles
    // - Female wrestlers can hold Female and Mixed titles  
    // - Other gender wrestlers can hold any title
    let gender_filter = match wrestler_gender {
        "Male" => vec!["Male", "Mixed"],
        "Female" => vec!["Female", "Mixed"], 
        _ => vec!["Male", "Female", "Mixed"], // "Other" or any other gender
    };
    
    // Get active titles that match gender criteria
    let filtered_titles = titles::table
        .filter(titles::is_active.eq(true))
        .filter(titles::gender.eq_any(gender_filter))
        .order(titles::prestige_tier.asc())
        .then_order_by(titles::name.asc())
        .load::<Title>(conn)?;

    let mut titles_with_holders = Vec::new();

    for title in filtered_titles {
        // Get current holders for this title
        let current_holders_data = title_holders::table
            .inner_join(wrestlers::table.on(title_holders::wrestler_id.eq(wrestlers::id)))
            .filter(title_holders::title_id.eq(title.id))
            .filter(title_holders::held_until.is_null())
            .select((TitleHolder::as_select(), wrestlers::name, wrestlers::gender))
            .load::<(TitleHolder, String, String)>(conn)?;

        let current_holders: Vec<TitleHolderInfo> = current_holders_data
            .into_iter()
            .map(|(holder, wrestler_name, wrestler_gender)| TitleHolderInfo {
                holder,
                wrestler_name,
                wrestler_gender,
            })
            .collect();

        // Calculate days held for the first holder (for single titles)
        let days_held = if let Some(first_holder) = current_holders.first() {
            let now = Utc::now().naive_utc();
            let duration = now - first_holder.holder.held_since;
            Some(duration.num_days() as i32)
        } else {
            None
        };

        titles_with_holders.push(TitleWithHolders {
            title,
            current_holders,
            days_held,
        });
    }

    Ok(titles_with_holders)
}

/// Updates title holder (ends current reign and starts new one)
/// 
/// # Arguments
/// * `conn` - Mutable reference to the database connection
/// * `title_id` - ID of the title to update
/// * `new_wrestler_id` - ID of the new champion
/// * `event_name` - Optional event name where title changed hands
/// * `event_location` - Optional event location
/// * `change_method` - Optional method of title change (e.g., "Pinfall", "Submission")
/// 
/// # Returns
/// * `Ok(())` - If title holder was successfully updated
/// * `Err(DieselError)` - Database error if update fails
/// 
/// # Note
/// Validates string lengths to prevent database abuse (max 255 chars)
pub fn internal_update_title_holder(
    conn: &mut SqliteConnection,
    title_id: i32,
    new_wrestler_id: i32,
    event_name: Option<&str>,
    event_location: Option<&str>,
    change_method: Option<&str>,
) -> Result<(), DieselError> {
    use crate::schema::title_holders;
    use diesel::result::{DatabaseErrorKind, Error as DieselError};

    // Input validation to prevent abuse
    const MAX_STRING_LENGTH: usize = 255;
    
    if let Some(name) = event_name {
        if name.len() > MAX_STRING_LENGTH {
            return Err(DieselError::DatabaseError(
                DatabaseErrorKind::Unknown,
                Box::new("Event name too long".to_string())
            ));
        }
    }
    
    if let Some(location) = event_location {
        if location.len() > MAX_STRING_LENGTH {
            return Err(DieselError::DatabaseError(
                DatabaseErrorKind::Unknown,
                Box::new("Event location too long".to_string())
            ));
        }
    }
    
    if let Some(method) = change_method {
        if method.len() > MAX_STRING_LENGTH {
            return Err(DieselError::DatabaseError(
                DatabaseErrorKind::Unknown,
                Box::new("Change method too long".to_string())
            ));
        }
    }

    let now = Utc::now().naive_utc();

    // End current title reigns for this title
    diesel::update(title_holders::table)
        .filter(title_holders::title_id.eq(title_id))
        .filter(title_holders::held_until.is_null())
        .set(title_holders::held_until.eq(now))
        .execute(conn)?;

    // Create new title holder record
    let new_holder = NewTitleHolder {
        title_id,
        wrestler_id: new_wrestler_id,
        held_since: now,
        event_name: event_name.map(|s| s.to_string()),
        event_location: event_location.map(|s| s.to_string()),
        change_method: change_method.map(|s| s.to_string()),
    };

    diesel::insert_into(title_holders::table)
        .values(&new_holder)
        .execute(conn)?;

    Ok(())
}

/// Tauri command to change a championship title holder
/// 
/// # Arguments
/// * `state` - The Tauri state containing the database pool
/// * `title_id` - ID of the title to update
/// * `new_wrestler_id` - ID of the new champion
/// * `event_name` - Optional event name
/// * `event_location` - Optional event location
/// * `change_method` - Optional method of victory
/// 
/// # Returns
/// * `Ok(String)` - Success message
/// * `Err(String)` - Error message if update fails
#[tauri::command]
pub fn update_title_holder(
    state: State<'_, DbState>,
    title_id: i32,
    new_wrestler_id: i32,
    event_name: Option<String>,
    event_location: Option<String>,
    change_method: Option<String>,
) -> Result<String, String> {
    let mut conn = get_connection(&state)?;

    internal_update_title_holder(
        &mut conn,
        title_id,
        new_wrestler_id,
        event_name.as_deref(),
        event_location.as_deref(),
        change_method.as_deref(),
    )
    .map_err(|e| {
        error!("Error updating title holder: {}", e);
        format!("Failed to update title holder: {}", e)
    })?;

    Ok("Title holder updated successfully".to_string())
}

/// Tauri command to delete a championship title
/// 
/// # Arguments
/// * `state` - The Tauri state containing the database pool
/// * `title_id` - ID of the title to delete
/// 
/// # Returns
/// * `Ok(String)` - Success message
/// * `Err(String)` - Error message if deletion fails or title is system-created
#[tauri::command]
pub fn delete_title(state: State<'_, DbState>, title_id: i32) -> Result<String, String> {
    let mut conn = get_connection(&state)?;

    internal_delete_title(&mut conn, title_id)
        .inspect(|_| {
            info!("Title with ID {} deleted successfully", title_id);
        })
        .map_err(|e| {
            error!("Error deleting title: {}", e);
            match e {
                DieselError::RollbackTransaction => "Cannot delete system title - only user-created titles can be deleted".to_string(),
                DieselError::NotFound => "Title not found".to_string(),
                _ => format!("Failed to delete title: {}", e),
            }
        })
        .map(|_| "Title deleted successfully".to_string())
}

/// Tauri command to get titles compatible with a wrestler's gender
/// 
/// # Arguments
/// * `state` - The Tauri state containing the database pool
/// * `wrestler_gender` - Gender of the wrestler
/// 
/// # Returns
/// * `Ok(Vec<TitleWithHolders>)` - Vector of compatible titles
/// * `Err(String)` - Error message if query fails
#[tauri::command]
pub fn get_titles_for_wrestler(
    state: State<'_, DbState>,
    wrestler_gender: String,
) -> Result<Vec<TitleWithHolders>, String> {
    let mut conn = get_connection(&state)?;
    
    internal_get_titles_for_wrestler_gender(&mut conn, &wrestler_gender)
        .map_err(|e| {
            error!("Error fetching titles for wrestler gender: {}", e);
            format!("Failed to fetch titles for wrestler: {}", e)
        })
}

/// Gets all titles assigned to a specific show
/// 
/// # Arguments
/// * `conn` - Mutable reference to the database connection
/// * `show_id` - ID of the show to filter by
/// 
/// # Returns
/// * `Ok(Vec<TitleWithHolders>)` - Vector of titles assigned to the show
/// * `Err(DieselError)` - Database error if query fails
pub fn internal_get_titles_for_show(
    conn: &mut SqliteConnection,
    show_id: i32,
) -> Result<Vec<TitleWithHolders>, DieselError> {
    use crate::schema::{titles, title_holders, wrestlers};
    
    // Get titles assigned to this specific show
    let all_titles = titles::table
        .filter(titles::is_active.eq(true))
        .filter(titles::show_id.eq(show_id))
        .order(titles::prestige_tier.asc())
        .then_order_by(titles::name.asc())
        .load::<Title>(conn)?;

    let mut titles_with_holders = Vec::new();

    for title in all_titles {
        // Get current holders for this title
        let current_holders_data = title_holders::table
            .inner_join(wrestlers::table.on(title_holders::wrestler_id.eq(wrestlers::id)))
            .filter(title_holders::title_id.eq(title.id))
            .filter(title_holders::held_until.is_null())
            .select((TitleHolder::as_select(), wrestlers::name, wrestlers::gender))
            .load::<(TitleHolder, String, String)>(conn)?;

        let current_holders: Vec<TitleHolderInfo> = current_holders_data
            .into_iter()
            .map(|(holder, wrestler_name, wrestler_gender)| TitleHolderInfo {
                holder,
                wrestler_name,
                wrestler_gender,
            })
            .collect();

        // Calculate days held for the first holder (for single titles)
        let days_held = if let Some(first_holder) = current_holders.first() {
            let now = Utc::now().naive_utc();
            let duration = now - first_holder.holder.held_since;
            Some(duration.num_days() as i32)
        } else {
            None
        };

        titles_with_holders.push(TitleWithHolders {
            title,
            current_holders,
            days_held,
        });
    }

    Ok(titles_with_holders)
}

/// Gets all unassigned titles (not assigned to any show)
/// 
/// # Arguments
/// * `conn` - Mutable reference to the database connection
/// 
/// # Returns
/// * `Ok(Vec<TitleWithHolders>)` - Vector of cross-brand titles
/// * `Err(DieselError)` - Database error if query fails
pub fn internal_get_unassigned_titles(
    conn: &mut SqliteConnection,
) -> Result<Vec<TitleWithHolders>, DieselError> {
    use crate::schema::{titles, title_holders, wrestlers};
    
    // Get titles not assigned to any show
    let all_titles = titles::table
        .filter(titles::is_active.eq(true))
        .filter(titles::show_id.is_null())
        .order(titles::prestige_tier.asc())
        .then_order_by(titles::name.asc())
        .load::<Title>(conn)?;

    let mut titles_with_holders = Vec::new();

    for title in all_titles {
        // Get current holders for this title
        let current_holders_data = title_holders::table
            .inner_join(wrestlers::table.on(title_holders::wrestler_id.eq(wrestlers::id)))
            .filter(title_holders::title_id.eq(title.id))
            .filter(title_holders::held_until.is_null())
            .select((TitleHolder::as_select(), wrestlers::name, wrestlers::gender))
            .load::<(TitleHolder, String, String)>(conn)?;

        let current_holders: Vec<TitleHolderInfo> = current_holders_data
            .into_iter()
            .map(|(holder, wrestler_name, wrestler_gender)| TitleHolderInfo {
                holder,
                wrestler_name,
                wrestler_gender,
            })
            .collect();

        // Calculate days held for the first holder (for single titles)
        let days_held = if let Some(first_holder) = current_holders.first() {
            let now = Utc::now().naive_utc();
            let duration = now - first_holder.holder.held_since;
            Some(duration.num_days() as i32)
        } else {
            None
        };

        titles_with_holders.push(TitleWithHolders {
            title,
            current_holders,
            days_held,
        });
    }

    Ok(titles_with_holders)
}

/// Tauri command to get titles assigned to a specific show
/// 
/// # Arguments
/// * `state` - The Tauri state containing the database pool
/// * `show_id` - ID of the show
/// 
/// # Returns
/// * `Ok(Vec<TitleWithHolders>)` - Vector of titles for the show
/// * `Err(String)` - Error message if query fails
#[tauri::command]
pub fn get_titles_for_show(
    state: State<'_, DbState>,
    show_id: i32,
) -> Result<Vec<TitleWithHolders>, String> {
    let mut conn = get_connection(&state)?;
    
    internal_get_titles_for_show(&mut conn, show_id)
        .map_err(|e| {
            error!("Error fetching titles for show: {}", e);
            format!("Failed to fetch titles for show: {}", e)
        })
}

/// Tauri command to get cross-brand titles not assigned to any show
/// 
/// # Arguments
/// * `state` - The Tauri state containing the database pool
/// 
/// # Returns
/// * `Ok(Vec<TitleWithHolders>)` - Vector of unassigned titles
/// * `Err(String)` - Error message if query fails
#[tauri::command]
pub fn get_unassigned_titles(
    state: State<'_, DbState>,
) -> Result<Vec<TitleWithHolders>, String> {
    let mut conn = get_connection(&state)?;
    
    internal_get_unassigned_titles(&mut conn)
        .map_err(|e| {
            error!("Error fetching unassigned titles: {}", e);
            format!("Failed to fetch unassigned titles: {}", e)
        })
}

/// Tauri command to create comprehensive test data for development
/// 
/// Creates the following test data:
/// - 2 shows (Monday Night RAW, Friday Night SmackDown)
/// - 5 wrestlers with detailed profiles and signature moves
/// - 15 championship titles across different tiers
/// - Show roster assignments
/// - Title holders
/// - Sample matches with participants
/// 
/// # Arguments
/// * `state` - The Tauri state containing the database pool
/// 
/// # Returns
/// * `Ok(String)` - Summary of created test data
/// * `Err(String)` - Error message if creation fails or data already exists
#[tauri::command]
pub fn create_test_data(state: State<'_, DbState>) -> Result<String, String> {
    let mut conn = get_connection(&state)?;
    
    // Check if specific test data already exists
    let existing_shows = internal_get_shows(&mut conn).map_err(|e| format!("Error checking shows: {}", e))?;
    
    // Check for specific test shows instead of any shows
    let test_show_names = ["Monday Night RAW", "Friday Night SmackDown"];
    let existing_test_shows: Vec<_> = existing_shows.iter()
        .filter(|show| test_show_names.contains(&show.name.as_str()))
        .collect();
    
    if !existing_test_shows.is_empty() {
        return Ok(format!("Test data already exists: found {} test shows", existing_test_shows.len()));
    }
    
    // Create test shows
    let test_shows = vec![
        ("Monday Night RAW", "WWE's flagship weekly show featuring the biggest superstars"),
        ("Friday Night SmackDown", "The longest-running weekly episodic TV show in history"),
    ];
    
    for (name, description) in test_shows {
        internal_create_show(&mut conn, name, description)
            .map_err(|e| format!("Failed to create show '{}': {}", name, e))?;
    }
    
    // Create test wrestlers with detailed information
    let test_wrestlers = vec![
        (
            "The Rock", "Dwayne Johnson", "The People's Champion", "Male", 245, 67,
            "6'5\"", "260 lbs", 1996, "WWE", 9, 6, 7, 9, 10, 8,
            "The Rock is one of the most electrifying superstars in sports entertainment history. Known for his incredible charisma, devastating finishing moves, and ability to captivate audiences worldwide. From his days as 'Rocky Maivia' to becoming 'The People's Champion,' The Rock has dominated both the wrestling ring and Hollywood.",
            true // This wrestler is user-created for testing modifications
        ),
        (
            "Stone Cold Steve Austin", "Steven James Anderson", "The Texas Rattlesnake", "Male", 312, 89,
            "6'2\"", "252 lbs", 1989, "WWE", 8, 7, 6, 8, 9, 7,
            "Stone Cold Steve Austin is the beer-drinking, hell-raising anti-hero who defined the Attitude Era. With his rebellious nature and iconic catchphrases, Austin became the face of WWE during its most successful period.",
            false
        ),
        (
            "Becky Lynch", "Rebecca Quin", "The Man", "Female", 156, 43,
            "5'6\"", "135 lbs", 2013, "WWE", 7, 8, 9, 8, 9, 8,
            "Becky Lynch transformed from 'The Irish Lass Kicker' to 'The Man' - the top superstar in all of WWE. Her journey from underdog to champion inspired millions and redefined what it means to be a top star in sports entertainment.",
            false
        ),
        (
            "Charlotte Flair", "Ashley Elizabeth Fliehr", "The Queen", "Female", 198, 52,
            "5'10\"", "143 lbs", 2012, "WWE", 7, 8, 8, 8, 9, 9,
            "Charlotte Flair is a second-generation superstar who has established herself as one of the most dominant competitors in WWE history. The daughter of 'Nature Boy' Ric Flair, she has carved out her own legendary legacy.",
            false
        ),
        (
            "John Cena", "John Felix Anthony Cena Jr.", "The Cenation Leader", "Male", 289, 78,
            "6'1\"", "251 lbs", 2002, "WWE", 8, 7, 7, 9, 10, 8,
            "John Cena is a 16-time World Champion who became the face of WWE for over a decade. Known for his 'Never Give Up' attitude and incredible work ethic, Cena has inspired millions of fans worldwide while also pursuing a successful acting career.",
            false
        ),
    ];
    
    for (name, real_name, nickname, gender, wins, losses, height, weight, debut_year, _unused_promotion, strength, speed, agility, stamina, charisma, technique, biography, is_user_created) in test_wrestlers {
        let wrestler = internal_create_enhanced_wrestler(
            &mut conn, name, real_name, nickname, gender, wins, losses, 
            height, weight, debut_year, strength, speed, agility, 
            stamina, charisma, technique, biography, is_user_created
        ).map_err(|e| format!("Failed to create wrestler '{}': {}", name, e))?;
        
        // Add signature moves for each wrestler
        let moves = match name {
            "The Rock" => vec![
                ("Rock Bottom", "primary"),
                ("People's Elbow", "primary"), 
                ("Samoan Drop", "secondary"),
                ("Spinebuster", "secondary"),
            ],
            "Stone Cold Steve Austin" => vec![
                ("Stone Cold Stunner", "primary"),
                ("Lou Thesz Press", "secondary"),
                ("Slingshot Suplex", "secondary"),
            ],
            "Becky Lynch" => vec![
                ("Dis-arm-her", "primary"),
                ("Manhandle Slam", "primary"),
                ("Bex-Ploder", "secondary"),
                ("Leg Drop", "secondary"),
            ],
            "Charlotte Flair" => vec![
                ("Figure Eight", "primary"),
                ("Natural Selection", "primary"),
                ("Big Boot", "secondary"),
                ("Spear", "secondary"),
            ],
            "John Cena" => vec![
                ("Attitude Adjustment", "primary"),
                ("STFU/STF", "primary"),
                ("Five Knuckle Shuffle", "secondary"),
                ("Flying Shoulder Block", "secondary"),
            ],
            _ => vec![],
        };
        
        for (move_name, move_type) in moves {
            internal_create_signature_move(&mut conn, wrestler.id, move_name, move_type)
                .map_err(|e| format!("Failed to create move '{}' for '{}': {}", move_name, name, e))?;
        }
    }
    
    // Create test titles
    let all_shows = internal_get_shows(&mut conn).map_err(|e| format!("Error getting shows: {}", e))?;
    let raw_show = all_shows.iter()
        .find(|show| show.name == "Monday Night RAW");
    let smackdown_show = all_shows.iter()
        .find(|show| show.name == "Friday Night SmackDown");

    let test_titles = vec![
        // Tier 1 - World Championships
        ("World Heavyweight Championship", "Singles", "World", "Male", raw_show.map(|s| s.id)),
        ("WWE Championship", "Singles", "WWE Championship", "Male", smackdown_show.map(|s| s.id)),
        ("Women's World Championship", "Singles", "Women's World", "Female", raw_show.map(|s| s.id)),
        ("WWE Women's Championship", "Singles", "WWE Women's Championship", "Female", smackdown_show.map(|s| s.id)),
        
        // Tier 2 - Secondary Championships
        ("Intercontinental Championship", "Singles", "Intercontinental", "Male", raw_show.map(|s| s.id)),
        ("United States Championship", "Singles", "United States", "Male", smackdown_show.map(|s| s.id)),
        ("Women's Intercontinental Championship", "Singles", "Women's Intercontinental", "Female", raw_show.map(|s| s.id)),
        ("Women's United States Championship", "Singles", "Women's United States", "Female", smackdown_show.map(|s| s.id)),
        
        // Tier 3 - Tag Team Championships
        ("World Tag Team Championship", "Tag Team", "World Tag Team", "Male", raw_show.map(|s| s.id)),
        ("WWE Tag Team Championship", "Tag Team", "WWE Tag Team", "Male", smackdown_show.map(|s| s.id)),
        ("Women's Tag Team Championship", "Tag Team", "Women's Tag Team", "Female", None), // Cross-brand
        
        // Tier 4 - Specialty Championships
        ("Money in the Bank", "Singles", "Money in the Bank", "Mixed", None),
        ("Hardcore Championship", "Singles", "Hardcore", "Mixed", None),
        ("Speed Championship", "Singles", "Speed", "Mixed", None),
        ("24/7 Championship", "Singles", "24/7", "Mixed", None),
    ];

    let mut title_count = 0;
    for (name, title_type, division, gender, show_id) in test_titles {
        internal_create_belt(&mut conn, name, title_type, division, gender, show_id, None, false)
            .map_err(|e| format!("Failed to create title '{}': {}", name, e))?;
        title_count += 1;
    }
    
    // Assign wrestlers to show rosters
    let all_wrestlers = internal_get_wrestlers(&mut conn).map_err(|e| format!("Error getting wrestlers: {}", e))?;
    let raw_show_id = raw_show.map(|s| s.id).ok_or("RAW show not found")?;
    let smackdown_show_id = smackdown_show.map(|s| s.id).ok_or("SmackDown show not found")?;
    
    // Assign all 5 wrestlers to RAW
    for wrestler in &all_wrestlers {
        internal_assign_wrestler_to_show(&mut conn, raw_show_id, wrestler.id)
            .map_err(|e| format!("Failed to assign wrestler {} to RAW: {}", wrestler.name, e))?;
    }
    
    // Assign 3 wrestlers to SmackDown (Charlotte, Becky, and Stone Cold)
    let smackdown_wrestlers = ["Charlotte Flair", "Becky Lynch", "Stone Cold Steve Austin"];
    for wrestler in &all_wrestlers {
        if smackdown_wrestlers.contains(&wrestler.name.as_str()) {
            internal_assign_wrestler_to_show(&mut conn, smackdown_show_id, wrestler.id)
                .map_err(|e| format!("Failed to assign wrestler {} to SmackDown: {}", wrestler.name, e))?;
        }
    }
    
    // Assign title holders
    let all_titles = internal_get_titles(&mut conn).map_err(|e| format!("Error getting titles: {}", e))?;
    
    // Make The Rock the World Heavyweight Champion
    if let Some(rock) = all_wrestlers.iter().find(|w| w.name == "The Rock") {
        if let Some(whc) = all_titles.iter().find(|t| t.title.name == "World Heavyweight Championship") {
            internal_update_title_holder(
                &mut conn,
                whc.title.id,
                rock.id,
                Some("Monday Night RAW"),
                Some("Won in tournament final"),
                None
            ).map_err(|e| format!("Failed to assign World Heavyweight Championship: {}", e))?;
        }
    }
    
    // Make Charlotte Flair the WWE Women's Champion
    if let Some(charlotte) = all_wrestlers.iter().find(|w| w.name == "Charlotte Flair") {
        if let Some(wwe_womens) = all_titles.iter().find(|t| t.title.name == "WWE Women's Championship") {
            internal_update_title_holder(
                &mut conn,
                wwe_womens.title.id,
                charlotte.id,
                Some("Friday Night SmackDown"),
                Some("Defeated previous champion"),
                None
            ).map_err(|e| format!("Failed to assign WWE Women's Championship: {}", e))?;
        }
    }
    
    // Create sample matches
    let match_data_list = vec![
        // RAW matches
        MatchData {
            show_id: raw_show_id,
            match_name: Some("World Heavyweight Championship Match".to_string()),
            match_type: "Singles".to_string(),
            match_stipulation: Some("Standard".to_string()),
            scheduled_date: None,
            match_order: Some(5),
            is_title_match: true,
            title_id: all_titles.iter().find(|t| t.title.name == "World Heavyweight Championship").map(|t| t.title.id),
        },
        MatchData {
            show_id: raw_show_id,
            match_name: Some("Grudge Match".to_string()),
            match_type: "Singles".to_string(),
            match_stipulation: Some("No Disqualification".to_string()),
            scheduled_date: None,
            match_order: Some(3),
            is_title_match: false,
            title_id: None,
        },
        MatchData {
            show_id: raw_show_id,
            match_name: Some("Opening Contest".to_string()),
            match_type: "Singles".to_string(),
            match_stipulation: Some("Standard".to_string()),
            scheduled_date: None,
            match_order: Some(1),
            is_title_match: false,
            title_id: None,
        },
        // SmackDown matches
        MatchData {
            show_id: smackdown_show_id,
            match_name: Some("WWE Women's Championship Match".to_string()),
            match_type: "Singles".to_string(),
            match_stipulation: Some("Standard".to_string()),
            scheduled_date: None,
            match_order: Some(4),
            is_title_match: true,
            title_id: all_titles.iter().find(|t| t.title.name == "WWE Women's Championship").map(|t| t.title.id),
        },
        MatchData {
            show_id: smackdown_show_id,
            match_name: Some("Main Event Singles Match".to_string()),
            match_type: "Singles".to_string(),
            match_stipulation: Some("Falls Count Anywhere".to_string()),
            scheduled_date: None,
            match_order: Some(5),
            is_title_match: false,
            title_id: None,
        },
    ];
    
    let mut match_count = 0;
    for match_data in match_data_list {
        let created_match = internal_create_match(&mut conn, &match_data)
            .map_err(|e| format!("Failed to create match '{}': {}", match_data.match_name.as_deref().unwrap_or("Unknown"), e))?;
        
        // Add participants based on match
        match match_data.match_name.as_deref().unwrap_or("") {
            "World Heavyweight Championship Match" => {
                // The Rock vs John Cena
                if let Some(rock) = all_wrestlers.iter().find(|w| w.name == "The Rock") {
                    internal_add_wrestler_to_match(&mut conn, created_match.id, rock.id, None, Some(1))
                        .map_err(|e| format!("Failed to add The Rock to match: {}", e))?;
                }
                if let Some(cena) = all_wrestlers.iter().find(|w| w.name == "John Cena") {
                    internal_add_wrestler_to_match(&mut conn, created_match.id, cena.id, None, Some(2))
                        .map_err(|e| format!("Failed to add John Cena to match: {}", e))?;
                }
                // Set The Rock as winner
                if let Some(rock) = all_wrestlers.iter().find(|w| w.name == "The Rock") {
                    internal_set_match_winner(&mut conn, created_match.id, rock.id)
                        .map_err(|e| format!("Failed to set match winner: {}", e))?;
                }
            },
            "Grudge Match" => {
                // Stone Cold vs The Rock
                if let Some(austin) = all_wrestlers.iter().find(|w| w.name == "Stone Cold Steve Austin") {
                    internal_add_wrestler_to_match(&mut conn, created_match.id, austin.id, None, Some(1))
                        .map_err(|e| format!("Failed to add Stone Cold to match: {}", e))?;
                }
                if let Some(rock) = all_wrestlers.iter().find(|w| w.name == "The Rock") {
                    internal_add_wrestler_to_match(&mut conn, created_match.id, rock.id, None, Some(2))
                        .map_err(|e| format!("Failed to add The Rock to match: {}", e))?;
                }
            },
            "Opening Contest" => {
                // Becky Lynch vs John Cena (intergender match)
                if let Some(becky) = all_wrestlers.iter().find(|w| w.name == "Becky Lynch") {
                    internal_add_wrestler_to_match(&mut conn, created_match.id, becky.id, None, Some(1))
                        .map_err(|e| format!("Failed to add Becky Lynch to match: {}", e))?;
                }
                if let Some(cena) = all_wrestlers.iter().find(|w| w.name == "John Cena") {
                    internal_add_wrestler_to_match(&mut conn, created_match.id, cena.id, None, Some(2))
                        .map_err(|e| format!("Failed to add John Cena to match: {}", e))?;
                }
            },
            "WWE Women's Championship Match" => {
                // Charlotte vs Becky
                if let Some(charlotte) = all_wrestlers.iter().find(|w| w.name == "Charlotte Flair") {
                    internal_add_wrestler_to_match(&mut conn, created_match.id, charlotte.id, None, Some(1))
                        .map_err(|e| format!("Failed to add Charlotte to match: {}", e))?;
                }
                if let Some(becky) = all_wrestlers.iter().find(|w| w.name == "Becky Lynch") {
                    internal_add_wrestler_to_match(&mut conn, created_match.id, becky.id, None, Some(2))
                        .map_err(|e| format!("Failed to add Becky to match: {}", e))?;
                }
                // Set Charlotte as winner
                if let Some(charlotte) = all_wrestlers.iter().find(|w| w.name == "Charlotte Flair") {
                    internal_set_match_winner(&mut conn, created_match.id, charlotte.id)
                        .map_err(|e| format!("Failed to set match winner: {}", e))?;
                }
            },
            "Main Event Singles Match" => {
                // Stone Cold vs Charlotte
                if let Some(austin) = all_wrestlers.iter().find(|w| w.name == "Stone Cold Steve Austin") {
                    internal_add_wrestler_to_match(&mut conn, created_match.id, austin.id, None, Some(1))
                        .map_err(|e| format!("Failed to add Stone Cold to match: {}", e))?;
                }
                if let Some(charlotte) = all_wrestlers.iter().find(|w| w.name == "Charlotte Flair") {
                    internal_add_wrestler_to_match(&mut conn, created_match.id, charlotte.id, None, Some(2))
                        .map_err(|e| format!("Failed to add Charlotte to match: {}", e))?;
                }
            },
            _ => {}
        }
        
        match_count += 1;
    }
    
    info!("Test data created successfully");
    Ok(format!("Test data created: 2 shows, 5 wrestlers, {} titles, show rosters assigned, 2 title holders, and {} matches with participants", title_count, match_count))
}

// ===== Show Roster Operations =====

/// Gets all wrestlers assigned to a specific show's roster
/// 
/// # Arguments
/// * `conn` - Mutable reference to the database connection
/// * `show_id` - ID of the show
/// 
/// # Returns
/// * `Ok(Vec<Wrestler>)` - Vector of wrestlers on the show's roster
/// * `Err(DieselError)` - Database error if query fails
/// 
/// # Note
/// Only returns active roster assignments
pub fn internal_get_wrestlers_for_show(
    conn: &mut SqliteConnection,
    show_id: i32,
) -> Result<Vec<Wrestler>, DieselError> {
    use crate::schema::{wrestlers, show_rosters};
    
    wrestlers::table
        .inner_join(show_rosters::table.on(wrestlers::id.eq(show_rosters::wrestler_id)))
        .filter(show_rosters::show_id.eq(show_id))
        .filter(show_rosters::is_active.eq(true))
        .select(Wrestler::as_select())
        .order(wrestlers::name.asc())
        .load::<Wrestler>(conn)
}

/// Gets all wrestlers not currently assigned to any show (internal function)
/// 
/// # Arguments
/// * `conn` - Mutable reference to the database connection
/// 
/// # Returns
/// * `Ok(Vec<Wrestler>)` - Vector of unassigned wrestlers
/// * `Err(DieselError)` - Database error if query fails
/// 
/// # Note
/// Uses LEFT JOIN to find wrestlers with no active show roster assignments
pub fn internal_get_unassigned_wrestlers(
    conn: &mut SqliteConnection,
) -> Result<Vec<Wrestler>, DieselError> {
    use crate::schema::{wrestlers, show_rosters};
    
    wrestlers::table
        .left_join(
            show_rosters::table.on(
                wrestlers::id.eq(show_rosters::wrestler_id)
                    .and(show_rosters::is_active.eq(true))
            )
        )
        .filter(show_rosters::wrestler_id.is_null())
        .select(Wrestler::as_select())
        .order(wrestlers::name.asc())
        .load::<Wrestler>(conn)
}

/// Gets the current active show assignment for a wrestler
/// 
/// # Arguments
/// * `conn` - Mutable reference to the database connection
/// * `wrestler_id` - ID of the wrestler to check
/// 
/// # Returns
/// * `Ok(Option<ShowRoster>)` - The active assignment if found, None otherwise
/// * `Err(DieselError)` - Database error if query fails
pub fn internal_get_current_show_for_wrestler(
    conn: &mut SqliteConnection,
    wrestler_id: i32,
) -> Result<Option<ShowRoster>, DieselError> {
    use crate::schema::show_rosters;
    
    show_rosters::table
        .filter(show_rosters::wrestler_id.eq(wrestler_id))
        .filter(show_rosters::is_active.eq(true))
        .first::<ShowRoster>(conn)
        .optional()
}

/// Assigns a wrestler to a show's roster with exclusive assignment logic
/// 
/// # Arguments
/// * `conn` - Mutable reference to the database connection
/// * `show_id` - ID of the show
/// * `wrestler_id` - ID of the wrestler to assign
/// 
/// # Returns
/// * `Ok(())` - If assignment was successful or already exists
/// * `Err(DieselError)` - Database error if assignment fails
/// 
/// # Note
/// Implements exclusive assignment logic - wrestler can only be on one show at a time
/// Uses database transaction for atomicity
pub fn internal_assign_wrestler_to_show(
    conn: &mut SqliteConnection,
    show_id: i32,
    wrestler_id: i32,
) -> Result<(), DieselError> {
    use crate::schema::show_rosters;
    use chrono::Utc;
    
    // Use transaction for atomicity
    conn.transaction::<(), DieselError, _>(|conn| {
        // Check if wrestler is already assigned to the target show
        let existing_assignment = show_rosters::table
            .filter(show_rosters::show_id.eq(show_id))
            .filter(show_rosters::wrestler_id.eq(wrestler_id))
            .first::<ShowRoster>(conn)
            .optional()?;
        
        if let Some(assignment) = existing_assignment {
            if assignment.is_active {
                // Wrestler already assigned to this show and active - nothing to do
                info!("Wrestler {} already assigned to show {} (active)", wrestler_id, show_id);
                return Ok(());
            } else {
                // Reactivate the existing inactive assignment to this show
                info!("Reactivating existing assignment for wrestler {} to show {}", wrestler_id, show_id);
                diesel::update(show_rosters::table)
                    .filter(show_rosters::id.eq(assignment.id))
                    .set((
                        show_rosters::is_active.eq(true),
                        show_rosters::assigned_at.eq(Some(Utc::now().naive_utc())),
                    ))
                    .execute(conn)?;
                return Ok(());
            }
        }
        
        // Check if wrestler is currently assigned to ANY other show
        let current_assignment = internal_get_current_show_for_wrestler(conn, wrestler_id)?;
        
        if let Some(current) = current_assignment {
            if current.show_id == show_id {
                // Already assigned to target show (shouldn't happen due to check above, but defensive)
                info!("Wrestler {} already assigned to show {} (active)", wrestler_id, show_id);
                return Ok(());
            }
            
            // Deactivate current assignment (transfer)
            info!("Transferring wrestler {} from show {} to show {}", wrestler_id, current.show_id, show_id);
            diesel::update(show_rosters::table)
                .filter(show_rosters::id.eq(current.id))
                .set(show_rosters::is_active.eq(false))
                .execute(conn)?;
        } else {
            info!("Assigning wrestler {} to show {} (new assignment)", wrestler_id, show_id);
        }
        
        // Create new assignment to target show
        let new_assignment = NewShowRoster {
            show_id,
            wrestler_id,
            assigned_at: Some(Utc::now().naive_utc()),
            is_active: true,
        };
        
        diesel::insert_into(show_rosters::table)
            .values(&new_assignment)
            .execute(conn)?;
        
        Ok(())
    })
}

/// Removes a wrestler from a show's roster
/// 
/// # Arguments
/// * `conn` - Mutable reference to the database connection
/// * `show_id` - ID of the show
/// * `wrestler_id` - ID of the wrestler to remove
/// 
/// # Returns
/// * `Ok(())` - If removal was successful
/// * `Err(DieselError)` - Database error if removal fails
/// 
/// # Note
/// Sets is_active to false rather than deleting the record
pub fn internal_remove_wrestler_from_show(
    conn: &mut SqliteConnection,
    show_id: i32,
    wrestler_id: i32,
) -> Result<(), DieselError> {
    use crate::schema::show_rosters;
    
    diesel::update(show_rosters::table)
        .filter(show_rosters::show_id.eq(show_id))
        .filter(show_rosters::wrestler_id.eq(wrestler_id))
        .filter(show_rosters::is_active.eq(true))
        .set(show_rosters::is_active.eq(false))
        .execute(conn)?;
    
    Ok(())
}

/// Tauri command to get all wrestlers on a show's roster
/// 
/// # Arguments
/// * `state` - The Tauri state containing the database pool
/// * `show_id` - ID of the show
/// 
/// # Returns
/// * `Ok(Vec<Wrestler>)` - Vector of wrestlers on the roster
/// * `Err(String)` - Error message if query fails
#[tauri::command]
pub fn get_wrestlers_for_show(
    state: State<'_, DbState>,
    show_id: i32,
) -> Result<Vec<Wrestler>, String> {
    let mut conn = get_connection(&state)?;
    
    internal_get_wrestlers_for_show(&mut conn, show_id)
        .map_err(|e| {
            error!("Error loading wrestlers for show: {}", e);
            format!("Failed to load wrestlers for show: {}", e)
        })
}

/// Tauri command to assign a wrestler to a show's roster
/// 
/// # Arguments
/// * `state` - The Tauri state containing the database pool
/// * `show_id` - ID of the show
/// * `wrestler_id` - ID of the wrestler to assign
/// 
/// # Returns
/// * `Ok(String)` - Success message
/// * `Err(String)` - Error message if assignment fails
#[tauri::command]
pub fn assign_wrestler_to_show(
    state: State<'_, DbState>,
    show_id: i32,
    wrestler_id: i32,
) -> Result<String, String> {
    let mut conn = get_connection(&state)?;
    
    internal_assign_wrestler_to_show(&mut conn, show_id, wrestler_id)
        .map_err(|e| {
            error!("Error assigning wrestler to show: {}", e);
            format!("Failed to assign wrestler to show: {}", e)
        })
        .map(|_| "Wrestler assigned to show successfully".to_string())
}

/// Tauri command to remove a wrestler from a show's roster
/// 
/// # Arguments
/// * `state` - The Tauri state containing the database pool
/// * `show_id` - ID of the show
/// * `wrestler_id` - ID of the wrestler to remove
/// 
/// # Returns
/// * `Ok(String)` - Success message
/// * `Err(String)` - Error message if removal fails
#[tauri::command]
pub fn remove_wrestler_from_show(
    state: State<'_, DbState>,
    show_id: i32,
    wrestler_id: i32,
) -> Result<String, String> {
    let mut conn = get_connection(&state)?;
    
    internal_remove_wrestler_from_show(&mut conn, show_id, wrestler_id)
        .map_err(|e| {
            error!("Error removing wrestler from show: {}", e);
            format!("Failed to remove wrestler from show: {}", e)
        })
        .map(|_| "Wrestler removed from show successfully".to_string())
}

/// Internal function to get shows that a wrestler is currently assigned to
/// 
/// # Arguments
/// * `conn` - Mutable reference to the database connection
/// * `wrestler_id` - ID of the wrestler
/// 
/// # Returns
/// * `Ok(Vec<Show>)` - Vector of shows the wrestler is assigned to
/// * `Err(DieselError)` - Database error if query fails
/// 
/// # Note
/// Only returns shows with active roster assignments.
/// Result set is typically small (1-5 shows per wrestler in most cases).
/// Returns empty vector if wrestler has no active assignments.
/// 
/// # Examples
/// ```rust
/// let shows = internal_get_shows_for_wrestler(&mut conn, wrestler_id)?;
/// for show in shows {
///     println!("Wrestler is assigned to: {}", show.name);
/// }
/// ```
pub fn internal_get_shows_for_wrestler(
    conn: &mut SqliteConnection,
    wrestler_id: i32,
) -> Result<Vec<Show>, DieselError> {
    use crate::schema::{shows, show_rosters};
    
    shows::table
        .inner_join(show_rosters::table.on(shows::id.eq(show_rosters::show_id)))
        .filter(show_rosters::wrestler_id.eq(wrestler_id))
        .filter(show_rosters::is_active.eq(true))
        .select(Show::as_select())
        .order(shows::name.asc())
        .load::<Show>(conn)
}

/// Tauri command to get shows that a wrestler is currently assigned to
/// 
/// # Arguments
/// * `state` - The Tauri state containing the database pool
/// * `wrestler_id` - ID of the wrestler
/// 
/// # Returns
/// * `Ok(Vec<Show>)` - Vector of shows the wrestler is assigned to
/// * `Err(String)` - Error message if query fails
#[tauri::command]
pub fn get_shows_for_wrestler(
    state: State<'_, DbState>,
    wrestler_id: i32,
) -> Result<Vec<Show>, String> {
    // Input validation
    if wrestler_id <= 0 {
        error!("Invalid wrestler ID provided: {}", wrestler_id);
        return Err(format!("Invalid wrestler ID: {}", wrestler_id));
    }
    
    let mut conn = get_connection(&state)?;
    
    internal_get_shows_for_wrestler(&mut conn, wrestler_id)
        .map_err(|e| {
            error!("Error loading shows for wrestler {}: {}", wrestler_id, e);
            format!("Failed to load shows for wrestler {}: {}", wrestler_id, e)
        })
}

// ===== Match Booking Operations =====

/// Creates a new match for a show
/// 
/// # Arguments
/// * `conn` - Mutable reference to the database connection
/// * `match_data` - MatchData struct containing all match details
/// 
/// # Returns
/// * `Ok(Match)` - The newly created match
/// * `Err(DieselError)` - Database error if creation fails
/// 
/// # Note
/// Scheduled date should be in "YYYY-MM-DD" format
pub fn internal_create_match(
    conn: &mut SqliteConnection,
    match_data: &MatchData,
) -> Result<Match, DieselError> {
    use crate::schema::matches;
    use chrono::NaiveDate;
    
    // Parse the date string if provided
    let scheduled_date = match_data.scheduled_date.as_ref()
        .and_then(|date_str| NaiveDate::parse_from_str(date_str, "%Y-%m-%d").ok());
    
    let new_match = NewMatch {
        show_id: match_data.show_id,
        match_name: match_data.match_name.clone(),
        match_type: match_data.match_type.clone(),
        match_stipulation: match_data.match_stipulation.clone(),
        scheduled_date,
        match_order: match_data.match_order,
        winner_id: None, // Will be set later when match is concluded
        is_title_match: match_data.is_title_match,
        title_id: match_data.title_id,
    };
    
    diesel::insert_into(matches::table)
        .values(&new_match)
        .returning(Match::as_returning())
        .get_result(conn)
}

/// Gets all matches for a specific show
/// 
/// # Arguments
/// * `conn` - Mutable reference to the database connection
/// * `show_id` - ID of the show
/// 
/// # Returns
/// * `Ok(Vec<Match>)` - Vector of matches ordered by match order
/// * `Err(DieselError)` - Database error if query fails
pub fn internal_get_matches_for_show(
    conn: &mut SqliteConnection,
    show_id: i32,
) -> Result<Vec<Match>, DieselError> {
    use crate::schema::matches;
    
    matches::table
        .filter(matches::show_id.eq(show_id))
        .order(matches::match_order.asc())
        .then_order_by(matches::id.asc())
        .load::<Match>(conn)
}

/// Adds a wrestler as a participant in a match
/// 
/// # Arguments
/// * `conn` - Mutable reference to the database connection
/// * `match_id` - ID of the match
/// * `wrestler_id` - ID of the wrestler to add
/// * `team_number` - Optional team number for tag matches
/// * `entrance_order` - Optional entrance order
/// 
/// # Returns
/// * `Ok(MatchParticipant)` - The newly created match participant
/// * `Err(DieselError)` - Database error if addition fails
pub fn internal_add_wrestler_to_match(
    conn: &mut SqliteConnection,
    match_id: i32,
    wrestler_id: i32,
    team_number: Option<i32>,
    entrance_order: Option<i32>,
) -> Result<MatchParticipant, DieselError> {
    use crate::schema::match_participants;
    
    let new_participant = NewMatchParticipant {
        match_id,
        wrestler_id,
        team_number,
        entrance_order,
    };
    
    diesel::insert_into(match_participants::table)
        .values(&new_participant)
        .returning(MatchParticipant::as_returning())
        .get_result(conn)
}

/// Gets all participants for a specific match with wrestler details
/// 
/// # Arguments
/// * `conn` - Mutable reference to the database connection
/// * `match_id` - ID of the match
/// 
/// # Returns
/// * `Ok(Vec<(MatchParticipant, Wrestler)>)` - Vector of participants with wrestler data
/// * `Err(DieselError)` - Database error if query fails
pub fn internal_get_match_participants(
    conn: &mut SqliteConnection,
    match_id: i32,
) -> Result<Vec<(MatchParticipant, Wrestler)>, DieselError> {
    use crate::schema::{match_participants, wrestlers};
    
    match_participants::table
        .inner_join(wrestlers::table.on(match_participants::wrestler_id.eq(wrestlers::id)))
        .filter(match_participants::match_id.eq(match_id))
        .order(match_participants::entrance_order.asc())
        .then_order_by(match_participants::id.asc())
        .select((MatchParticipant::as_select(), Wrestler::as_select()))
        .load::<(MatchParticipant, Wrestler)>(conn)
}

/// Updates the winner of a match
/// 
/// # Arguments
/// * `conn` - Mutable reference to the database connection
/// * `match_id` - ID of the match
/// * `winner_id` - ID of the winning wrestler
/// 
/// # Returns
/// * `Ok(Match)` - The updated match with winner set
/// * `Err(DieselError)` - Database error if update fails
pub fn internal_set_match_winner(
    conn: &mut SqliteConnection,
    match_id: i32,
    winner_id: i32,
) -> Result<Match, DieselError> {
    use crate::schema::matches;
    
    diesel::update(matches::table)
        .filter(matches::id.eq(match_id))
        .set(matches::winner_id.eq(winner_id))
        .returning(Match::as_returning())
        .get_result(conn)
}

/// Tauri command to create a new match for booking
/// 
/// # Arguments
/// * `state` - The Tauri state containing the database pool
/// * `match_data` - MatchData struct with match details
/// 
/// # Returns
/// * `Ok(Match)` - The newly created match
/// * `Err(String)` - Error message if creation fails
#[tauri::command]
pub fn create_match(
    state: State<'_, DbState>,
    match_data: MatchData,
) -> Result<Match, String> {
    let mut conn = get_connection(&state)?;
    
    internal_create_match(&mut conn, &match_data)
        .map_err(|e| {
            error!("Error creating match: {}", e);
            format!("Failed to create match: {}", e)
        })
}

/// Tauri command to get all matches for a show
/// 
/// # Arguments
/// * `state` - The Tauri state containing the database pool
/// * `show_id` - ID of the show
/// 
/// # Returns
/// * `Ok(Vec<Match>)` - Vector of matches for the show
/// * `Err(String)` - Error message if query fails
#[tauri::command]
pub fn get_matches_for_show(
    state: State<'_, DbState>,
    show_id: i32,
) -> Result<Vec<Match>, String> {
    let mut conn = get_connection(&state)?;
    
    internal_get_matches_for_show(&mut conn, show_id)
        .map_err(|e| {
            error!("Error loading matches for show: {}", e);
            format!("Failed to load matches for show: {}", e)
        })
}

/// Tauri command to add a wrestler to a match
/// 
/// # Arguments
/// * `state` - The Tauri state containing the database pool
/// * `match_id` - ID of the match
/// * `wrestler_id` - ID of the wrestler to add
/// * `team_number` - Optional team assignment for tag matches
/// * `entrance_order` - Optional entrance order
/// 
/// # Returns
/// * `Ok(MatchParticipant)` - The created participant record
/// * `Err(String)` - Error message if addition fails
#[tauri::command]
pub fn add_wrestler_to_match(
    state: State<'_, DbState>,
    match_id: i32,
    wrestler_id: i32,
    team_number: Option<i32>,
    entrance_order: Option<i32>,
) -> Result<MatchParticipant, String> {
    let mut conn = get_connection(&state)?;
    
    internal_add_wrestler_to_match(&mut conn, match_id, wrestler_id, team_number, entrance_order)
        .map_err(|e| {
            error!("Error adding wrestler to match: {}", e);
            format!("Failed to add wrestler to match: {}", e)
        })
}

/// Tauri command to get all participants in a match
/// 
/// # Arguments
/// * `state` - The Tauri state containing the database pool
/// * `match_id` - ID of the match
/// 
/// # Returns
/// * `Ok(Vec<(MatchParticipant, Wrestler)>)` - Participants with wrestler details
/// * `Err(String)` - Error message if query fails
#[tauri::command]
pub fn get_match_participants(
    state: State<'_, DbState>,
    match_id: i32,
) -> Result<Vec<(MatchParticipant, Wrestler)>, String> {
    let mut conn = get_connection(&state)?;
    
    internal_get_match_participants(&mut conn, match_id)
        .map_err(|e| {
            error!("Error loading match participants: {}", e);
            format!("Failed to load match participants: {}", e)
        })
}

/// Tauri command to set the winner of a match
/// 
/// # Arguments
/// * `state` - The Tauri state containing the database pool
/// * `match_id` - ID of the match
/// * `winner_id` - ID of the winning wrestler
/// 
/// # Returns
/// * `Ok(Match)` - The updated match
/// * `Err(String)` - Error message if update fails
#[tauri::command]
pub fn set_match_winner(
    state: State<'_, DbState>,
    match_id: i32,
    winner_id: i32,
) -> Result<Match, String> {
    let mut conn = get_connection(&state)?;
    
    internal_set_match_winner(&mut conn, match_id, winner_id)
        .map_err(|e| {
            error!("Error setting match winner: {}", e);
            format!("Failed to set match winner: {}", e)
        })
}

/// Vacates a title by ending the current title reign
/// 
/// # Arguments
/// * `conn` - Mutable reference to the database connection
/// * `title_id` - ID of the title to vacate
/// * `event_name` - Optional event where title was vacated
/// * `event_location` - Optional event location
/// * `change_method` - Optional reason for vacancy
/// 
/// # Returns
/// * `Ok(())` - If title was successfully vacated
/// * `Err(DieselError)` - Database error if update fails
/// 
/// # Note
/// Validates string lengths to prevent database abuse
pub fn internal_vacate_title(
    conn: &mut SqliteConnection,
    title_id: i32,
    event_name: Option<&str>,
    event_location: Option<&str>,
    change_method: Option<&str>,
) -> Result<(), DieselError> {
    use crate::schema::title_holders;
    use diesel::result::{DatabaseErrorKind, Error as DieselError};
    
    // Input validation to prevent abuse
    const MAX_STRING_LENGTH: usize = 255;
    
    if let Some(name) = event_name {
        if name.len() > MAX_STRING_LENGTH {
            return Err(DieselError::DatabaseError(
                DatabaseErrorKind::Unknown,
                Box::new("Event name too long".to_string())
            ));
        }
    }
    
    if let Some(location) = event_location {
        if location.len() > MAX_STRING_LENGTH {
            return Err(DieselError::DatabaseError(
                DatabaseErrorKind::Unknown,
                Box::new("Event location too long".to_string())
            ));
        }
    }
    
    if let Some(method) = change_method {
        if method.len() > MAX_STRING_LENGTH {
            return Err(DieselError::DatabaseError(
                DatabaseErrorKind::Unknown,
                Box::new("Change method too long".to_string())
            ));
        }
    }
    
    let now = Utc::now().naive_utc();
    
    // End current title reigns for this title
    diesel::update(title_holders::table)
        .filter(title_holders::title_id.eq(title_id))
        .filter(title_holders::held_until.is_null())
        .set((
            title_holders::held_until.eq(now),
            title_holders::event_name.eq(event_name.map(|s| s.to_string())),
            title_holders::event_location.eq(event_location.map(|s| s.to_string())),
            title_holders::change_method.eq(change_method.map(|s| s.to_string())),
        ))
        .execute(conn)?;
    
    Ok(())
}

/// Tauri command to vacate a championship title
/// 
/// # Arguments
/// * `state` - The Tauri state containing the database pool
/// * `title_id` - ID of the title to vacate
/// * `event_name` - Optional event name
/// * `event_location` - Optional event location
/// * `change_method` - Optional vacancy reason (e.g., "Injury", "Retirement")
/// 
/// # Returns
/// * `Ok(String)` - Success message
/// * `Err(String)` - Error message if vacancy fails
#[tauri::command]
pub fn vacate_title(
    state: State<'_, DbState>,
    title_id: i32,
    event_name: Option<String>,
    event_location: Option<String>,
    change_method: Option<String>,
) -> Result<String, String> {
    let mut conn = get_connection(&state)?;
    
    internal_vacate_title(
        &mut conn,
        title_id,
        event_name.as_deref(),
        event_location.as_deref(),
        change_method.as_deref(),
    )
    .map_err(|e| {
        error!("Error vacating title: {}", e);
        format!("Failed to vacate title: {}", e)
    })?;
    
    Ok("Title vacated successfully".to_string())
}
