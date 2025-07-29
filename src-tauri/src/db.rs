use crate::models::{
    Match, MatchData, NewMatch, MatchParticipant, NewMatchParticipant,
    NewPromotion, NewShowRoster, NewShow, NewSignatureMove, NewTitle, NewTitleHolder, NewUser, NewWrestler, NewEnhancedWrestler, Promotion, PromotionData, ShowRoster, Show, ShowData, SignatureMove, Title, TitleData, TitleHolder, TitleWithHolders, TitleHolderInfo, User, UserData,
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
    promotion_id: i32,
) -> Result<Show, DieselError> {
    let new_show = NewShow {
        name: name.to_string(),
        description: description.to_string(),
        promotion_id,
    };

    diesel::insert_into(crate::schema::shows::dsl::shows)
        .values(&new_show)
        .returning(Show::as_returning())
        .get_result(conn)
}

/// Gets all shows for a promotion ordered by ID (used by tests and Tauri commands)
pub fn internal_get_shows(conn: &mut SqliteConnection, promo_id: i32) -> Result<Vec<Show>, DieselError> {
    use crate::schema::shows::dsl::*;
    shows
        .filter(promotion_id.eq(promo_id))
        .order(id.asc())
        .load::<Show>(conn)
}

#[tauri::command]
pub fn create_show(state: State<'_, DbState>, show_data: ShowData) -> Result<Show, String> {
    let mut conn = get_connection(&state)?;

    // TODO: Update to use actual promotion_id from frontend when CEO dashboard is implemented
    let default_promotion_id = 1;
    internal_create_show(&mut conn, &show_data.name, &show_data.description, default_promotion_id)
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

    // TODO: Update to use actual promotion_id from frontend when CEO dashboard is implemented
    let default_promotion_id = 1;
    internal_get_shows(&mut conn, default_promotion_id).map_err(|e| {
        error!("Error loading shows: {}", e);
        format!("Failed to load shows: {}", e)
    })
}

// ===== Promotion Operations =====

/// Creates a new promotion (used by tests and Tauri commands)
pub fn internal_create_promotion(
    conn: &mut SqliteConnection,
    name: &str,
    description: &str,
) -> Result<Promotion, DieselError> {
    let new_promotion = NewPromotion {
        name: name.to_string(),
        description: description.to_string(),
    };

    diesel::insert_into(crate::schema::promotions::dsl::promotions)
        .values(&new_promotion)
        .returning(Promotion::as_returning())
        .get_result(conn)
}

/// Gets all promotions from the database (used by tests and Tauri commands)
pub fn internal_get_promotions(conn: &mut SqliteConnection) -> Result<Vec<Promotion>, DieselError> {
    use crate::schema::promotions::dsl::*;
    
    promotions
        .order(name.asc())
        .select(Promotion::as_select())
        .load(conn)
}

#[tauri::command]
pub fn create_promotion(
    state: State<'_, DbState>,
    promotion_data: PromotionData,
) -> Result<Promotion, String> {
    let mut conn = get_connection(&state)?;

    internal_create_promotion(&mut conn, &promotion_data.name, &promotion_data.description)
        .map_err(|e| {
            error!("Error creating promotion: {}", e);
            format!("Failed to create promotion: {}", e)
        })
}

#[tauri::command]
pub fn get_promotions(state: State<'_, DbState>) -> Result<Vec<Promotion>, String> {
    let mut conn = get_connection(&state)?;

    internal_get_promotions(&mut conn).map_err(|e| {
        error!("Error loading promotions: {}", e);
        format!("Failed to load promotions: {}", e)
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

/// Gets all wrestlers ordered by ID (used by tests and Tauri commands)
pub fn internal_get_wrestlers(conn: &mut SqliteConnection) -> Result<Vec<Wrestler>, DieselError> {
    use crate::schema::wrestlers::dsl::*;
    wrestlers
        .order(id.asc())
        .load::<Wrestler>(conn)
}

/// Gets a specific wrestler by ID (used by tests and Tauri commands)
pub fn internal_get_wrestler_by_id(conn: &mut SqliteConnection, wrestler_id: i32) -> Result<Option<Wrestler>, DieselError> {
    use crate::schema::wrestlers::dsl::*;
    wrestlers.filter(id.eq(wrestler_id)).first::<Wrestler>(conn).optional()
}

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
        is_user_created: Some(false), // Default to system wrestler
    };

    diesel::insert_into(crate::schema::wrestlers::dsl::wrestlers)
        .values(&new_wrestler)
        .returning(Wrestler::as_returning())
        .get_result(conn)
}

/// Creates a new wrestler with enhanced details (used for test data)
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
    wrestler_promotion: &str,
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
        promotion: Some(wrestler_promotion.to_string()),
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
        promotion: wrestler_data.promotion.clone(),
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

/// Updates a wrestler's promotion
pub fn internal_update_wrestler_promotion(
    conn: &mut SqliteConnection,
    wrestler_id: i32,
    new_promotion: &str,
) -> Result<Wrestler, DieselError> {
    use crate::schema::wrestlers::dsl::*;
    
    diesel::update(wrestlers.filter(id.eq(wrestler_id)))
        .set(promotion.eq(new_promotion))
        .returning(Wrestler::as_returning())
        .get_result(conn)
}

/// Updates a wrestler's power ratings
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

/// Updates a wrestler's basic stats
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

/// Updates a wrestler's name and nickname
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

/// Updates a wrestler's biography
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

#[tauri::command]
pub fn get_wrestlers(state: State<'_, DbState>) -> Result<Vec<Wrestler>, String> {
    let mut conn = get_connection(&state)?;

    internal_get_wrestlers(&mut conn).map_err(|e| {
        error!("Error loading wrestlers: {}", e);
        format!("Failed to load wrestlers: {}", e)
    })
}

#[tauri::command]
pub fn get_wrestler_by_id(state: State<'_, DbState>, wrestler_id: i32) -> Result<Option<Wrestler>, String> {
    let mut conn = get_connection(&state)?;

    internal_get_wrestler_by_id(&mut conn, wrestler_id).map_err(|e| {
        error!("Error loading wrestler: {}", e);
        format!("Failed to load wrestler: {}", e)
    })
}

#[tauri::command]
pub fn update_wrestler_promotion(
    state: State<'_, DbState>,
    wrestler_id: i32,
    promotion: String,
) -> Result<Wrestler, String> {
    let mut conn = get_connection(&state)?;

    internal_update_wrestler_promotion(&mut conn, wrestler_id, &promotion)
        .inspect(|wrestler| {
            info!("Wrestler '{}' promotion updated to '{}'", wrestler.name, promotion);
        })
        .map_err(|e| {
            error!("Error updating wrestler promotion: {}", e);
            format!("Failed to update wrestler promotion: {}", e)
        })
}

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


// ===== Title Operations =====

/// Creates a new title/belt (used by tests and Tauri commands)
pub fn internal_create_belt(
    conn: &mut SqliteConnection,
    name: &str,
    title_type: &str,
    division: &str,
    gender: &str,
    show_id: Option<i32>,
    current_holder_id: Option<i32>,
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
    };

    diesel::insert_into(crate::schema::titles::dsl::titles)
        .values(&new_title)
        .returning(Title::as_returning())
        .get_result(conn)
}

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
    )
    .inspect(|title| {
        info!("Title '{}' created successfully", title.name);
    })
    .map_err(|e| {
        error!("Error creating title: {}", e);
        format!("Failed to create title: {}", e)
    })
}

/// Gets all titles with their current holders
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

#[tauri::command]
pub fn get_titles(state: State<'_, DbState>) -> Result<Vec<TitleWithHolders>, String> {
    let mut conn = get_connection(&state)?;
    
    internal_get_titles(&mut conn)
        .map_err(|e| {
            error!("Error fetching titles: {}", e);
            format!("Failed to fetch titles: {}", e)
        })
}

/// Updates title holder (ends current reign and starts new one)
pub fn internal_update_title_holder(
    conn: &mut SqliteConnection,
    title_id: i32,
    new_wrestler_id: i32,
    event_name: Option<&str>,
    event_location: Option<&str>,
    change_method: Option<&str>,
) -> Result<(), DieselError> {
    use crate::schema::title_holders;

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

/// Gets all titles filtered by show assignment
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

/// Creates test data if it doesn't already exist
#[tauri::command]
pub fn create_test_data(state: State<'_, DbState>) -> Result<String, String> {
    let mut conn = get_connection(&state)?;
    
    // Check if specific test data already exists
    let default_promotion_id = 1;
    let existing_shows = internal_get_shows(&mut conn, default_promotion_id).map_err(|e| format!("Error checking shows: {}", e))?;
    
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
        internal_create_show(&mut conn, name, description, default_promotion_id)
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
    
    for (name, real_name, nickname, gender, wins, losses, height, weight, debut_year, promotion, strength, speed, agility, stamina, charisma, technique, biography, is_user_created) in test_wrestlers {
        let wrestler = internal_create_enhanced_wrestler(
            &mut conn, name, real_name, nickname, gender, wins, losses, 
            height, weight, debut_year, promotion, strength, speed, agility, 
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
    let raw_show = internal_get_shows(&mut conn, default_promotion_id).map_err(|e| format!("Error getting shows: {}", e))?
        .into_iter()
        .find(|show| show.name == "Monday Night RAW");
    let smackdown_show = internal_get_shows(&mut conn, default_promotion_id).map_err(|e| format!("Error getting shows: {}", e))?
        .into_iter()
        .find(|show| show.name == "Friday Night SmackDown");

    let test_titles = vec![
        // Tier 1 - World Championships
        ("World Heavyweight Championship", "Singles", "World", "Male", raw_show.as_ref().map(|s| s.id)),
        ("WWE Championship", "Singles", "WWE Championship", "Male", smackdown_show.as_ref().map(|s| s.id)),
        ("Women's World Championship", "Singles", "Women's World", "Female", raw_show.as_ref().map(|s| s.id)),
        ("WWE Women's Championship", "Singles", "WWE Women's Championship", "Female", smackdown_show.as_ref().map(|s| s.id)),
        
        // Tier 2 - Secondary Championships
        ("Intercontinental Championship", "Singles", "Intercontinental", "Male", raw_show.as_ref().map(|s| s.id)),
        ("United States Championship", "Singles", "United States", "Male", smackdown_show.as_ref().map(|s| s.id)),
        ("Women's Intercontinental Championship", "Singles", "Women's Intercontinental", "Female", raw_show.as_ref().map(|s| s.id)),
        ("Women's United States Championship", "Singles", "Women's United States", "Female", smackdown_show.as_ref().map(|s| s.id)),
        
        // Tier 3 - Tag Team Championships
        ("World Tag Team Championship", "Tag Team", "World Tag Team", "Male", raw_show.as_ref().map(|s| s.id)),
        ("WWE Tag Team Championship", "Tag Team", "WWE Tag Team", "Male", smackdown_show.as_ref().map(|s| s.id)),
        ("Women's Tag Team Championship", "Tag Team", "Women's Tag Team", "Female", None), // Cross-brand
        
        // Tier 4 - Specialty Championships
        ("Money in the Bank", "Singles", "Money in the Bank", "Mixed", None),
        ("Hardcore Championship", "Singles", "Hardcore", "Mixed", None),
        ("Speed Championship", "Singles", "Speed", "Mixed", None),
        ("24/7 Championship", "Singles", "24/7", "Mixed", None),
    ];

    let mut title_count = 0;
    for (name, title_type, division, gender, show_id) in test_titles {
        internal_create_belt(&mut conn, name, title_type, division, gender, show_id, None)
            .map_err(|e| format!("Failed to create title '{}': {}", name, e))?;
        title_count += 1;
    }
    
    // Assign wrestlers to show rosters
    let all_wrestlers = internal_get_wrestlers(&mut conn).map_err(|e| format!("Error getting wrestlers: {}", e))?;
    let raw_show_id = raw_show.as_ref().map(|s| s.id).ok_or("RAW show not found")?;
    let smackdown_show_id = smackdown_show.as_ref().map(|s| s.id).ok_or("SmackDown show not found")?;
    
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

/// Gets all wrestlers assigned to a specific show
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

/// Assigns a wrestler to a show roster
pub fn internal_assign_wrestler_to_show(
    conn: &mut SqliteConnection,
    show_id: i32,
    wrestler_id: i32,
) -> Result<(), DieselError> {
    use crate::schema::show_rosters;
    use chrono::Utc;
    
    // Check if the assignment already exists and is active
    let existing = show_rosters::table
        .filter(show_rosters::show_id.eq(show_id))
        .filter(show_rosters::wrestler_id.eq(wrestler_id))
        .filter(show_rosters::is_active.eq(true))
        .first::<ShowRoster>(conn)
        .optional()?;
    
    if existing.is_some() {
        // Assignment already exists and is active
        return Ok(());
    }
    
    // Create new assignment
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
}

/// Removes a wrestler from a show roster (sets is_active to false)
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

// ===== Match Booking Operations =====

/// Creates a new match for a show
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

/// Adds a wrestler to a match
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

/// Gets all participants for a specific match
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
