use crate::models::{
    NewShow, NewSignatureMove, NewTitle, NewUser, NewWrestler, NewEnhancedWrestler, Show, ShowData, SignatureMove, Title, TitleData, User, UserData,
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

/// Gets all wrestlers ordered by ID (used by tests and Tauri commands)
pub fn internal_get_wrestlers(conn: &mut SqliteConnection) -> Result<Vec<Wrestler>, DieselError> {
    use crate::schema::wrestlers::dsl::*;
    wrestlers.order(id.asc()).load::<Wrestler>(conn)
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
    wrestler_trivia: &str,
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
        trivia: Some(wrestler_trivia.to_string()),
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

/// Creates test data if it doesn't already exist
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
            "The Rock is not only a wrestling legend but also one of the highest-paid actors in Hollywood! He's starred in major blockbuster films and has become a global icon beyond the wrestling world."
        ),
        (
            "Stone Cold Steve Austin", "Steven James Anderson", "The Texas Rattlesnake", "Male", 312, 89,
            "6'2\"", "252 lbs", 1989, "WWE", 8, 7, 6, 8, 9, 7,
            "Stone Cold Steve Austin is the beer-drinking, hell-raising anti-hero who defined the Attitude Era. With his rebellious nature and iconic catchphrases, Austin became the face of WWE during its most successful period.",
            "Austin's entrance music is one of the most recognizable in wrestling history, and his feuds with Mr. McMahon are considered among the greatest storylines ever told in sports entertainment."
        ),
        (
            "Becky Lynch", "Rebecca Quin", "The Man", "Female", 156, 43,
            "5'6\"", "135 lbs", 2013, "WWE", 7, 8, 9, 8, 9, 8,
            "Becky Lynch transformed from 'The Irish Lass Kicker' to 'The Man' - the top superstar in all of WWE. Her journey from underdog to champion inspired millions and redefined what it means to be a top star in sports entertainment.",
            "Becky was the first woman to main event WrestleMania when she headlined WrestleMania 35 in a historic Triple Threat match for both the Raw and SmackDown Women's Championships."
        ),
        (
            "Charlotte Flair", "Ashley Elizabeth Fliehr", "The Queen", "Female", 198, 52,
            "5'10\"", "143 lbs", 2012, "WWE", 7, 8, 8, 8, 9, 9,
            "Charlotte Flair is a second-generation superstar who has established herself as one of the most dominant competitors in WWE history. The daughter of 'Nature Boy' Ric Flair, she has carved out her own legendary legacy.",
            "Charlotte is a 14-time Women's Champion and has been featured in several historic firsts for women's wrestling, including the first women's Hell in a Cell match and the first women's main event on Monday Night Raw."
        ),
        (
            "John Cena", "John Felix Anthony Cena Jr.", "The Cenation Leader", "Male", 289, 78,
            "6'1\"", "251 lbs", 2002, "WWE", 8, 7, 7, 9, 10, 8,
            "John Cena is a 16-time World Champion who became the face of WWE for over a decade. Known for his 'Never Give Up' attitude and incredible work ethic, Cena has inspired millions of fans worldwide while also pursuing a successful acting career.",
            "Cena has granted over 650 Make-A-Wish requests, more than any other celebrity in the organization's history. His dedication to philanthropy matches his success in the ring."
        ),
    ];
    
    for (name, real_name, nickname, gender, wins, losses, height, weight, debut_year, promotion, strength, speed, agility, stamina, charisma, technique, biography, trivia) in test_wrestlers {
        let wrestler = internal_create_enhanced_wrestler(
            &mut conn, name, real_name, nickname, gender, wins, losses, 
            height, weight, debut_year, promotion, strength, speed, agility, 
            stamina, charisma, technique, biography, trivia
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
    
    info!("Test data created successfully");
    Ok("Test data created: 2 shows and 5 wrestlers".to_string())
}
