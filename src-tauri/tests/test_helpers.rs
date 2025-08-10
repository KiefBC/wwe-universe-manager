use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::sqlite::SqliteConnection;
use std::sync::Once;
use std::env;
use std::fs;

use wwe_universe_manager_lib::models::*;

static INIT: Once = Once::new();

pub fn setup_test_db() -> Pool<ConnectionManager<SqliteConnection>> {
    INIT.call_once(|| {
        env_logger::init();
    });
    
    // Always use isolated test database - never touch production database.db
    let test_db_path = "./test_database.db";
    
    // Remove existing test database to ensure fresh start
    if std::path::Path::new(test_db_path).exists() {
        fs::remove_file(test_db_path).ok();
    }
    
    // Set DATABASE_URL to test database for diesel commands
    env::set_var("DATABASE_URL", test_db_path);
    
    // Run migrations on fresh test database
    run_test_migrations(test_db_path);
    
    let manager = ConnectionManager::<SqliteConnection>::new(test_db_path);
    diesel::r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create test database connection pool")
}

fn run_test_migrations(database_path: &str) {
    // Create empty database file
    fs::File::create(database_path).expect("Failed to create test database file");
    
    // For tests, always use manual migration setup to avoid conflicts with existing migration history
    // This ensures we get a clean, isolated test database with the exact schema we need
    setup_test_schema_manually(database_path);
}

fn setup_test_schema_manually(database_path: &str) {
    // Fallback: Create connection and run migrations manually if diesel CLI is not available
    let mut conn = SqliteConnection::establish(database_path)
        .expect("Failed to establish connection to test database");
    
    // Run the 4 consolidated migrations manually
    // This ensures tests work even without diesel CLI
    run_consolidated_migrations(&mut conn);
    println!("Test database schema set up manually");
}


fn run_consolidated_migrations(conn: &mut SqliteConnection) {
    // Simplified migrations for testing - we'll skip complex triggers for now and focus on the core schema
    // This ensures we have a working test database without the complexity of trigger parsing
    
    // Migration 1: Create users table
    diesel::sql_query(r#"
        CREATE TABLE users (
            id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
            username TEXT NOT NULL,
            password TEXT NOT NULL,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )
    "#).execute(conn).expect("Failed to create users table");
    
    diesel::sql_query("CREATE UNIQUE INDEX idx_users_username ON users (username)")
        .execute(conn).expect("Failed to create users index");
    
    // Migration 2: Create wrestlers system
    diesel::sql_query(r#"
        CREATE TABLE wrestlers (
            id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
            name TEXT NOT NULL,
            gender TEXT NOT NULL,
            wins INTEGER NOT NULL DEFAULT 0,
            losses INTEGER NOT NULL DEFAULT 0,
            real_name TEXT,
            nickname TEXT,
            height TEXT,
            weight TEXT,
            debut_year INTEGER,
            strength INTEGER DEFAULT 5,
            speed INTEGER DEFAULT 5,
            agility INTEGER DEFAULT 5,
            stamina INTEGER DEFAULT 5,
            charisma INTEGER DEFAULT 5,
            technique INTEGER DEFAULT 5,
            biography TEXT,
            is_user_created BOOLEAN DEFAULT FALSE,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )
    "#).execute(conn).expect("Failed to create wrestlers table");

    diesel::sql_query(r#"
        CREATE TABLE signature_moves (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            wrestler_id INTEGER NOT NULL,
            move_name TEXT NOT NULL,
            move_type TEXT NOT NULL,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )
    "#).execute(conn).expect("Failed to create signature_moves table");

    diesel::sql_query("CREATE INDEX idx_signature_moves_wrestler_id ON signature_moves(wrestler_id)")
        .execute(conn).expect("Failed to create signature_moves index");
    
    // Migration 3: Create shows and titles system (core tables for testing)
    diesel::sql_query(r#"
        CREATE TABLE shows (
            id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
            name TEXT NOT NULL,
            description TEXT NOT NULL,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )
    "#).execute(conn).expect("Failed to create shows table");

    diesel::sql_query(r#"
        CREATE TABLE titles (
            id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
            name TEXT NOT NULL,
            current_holder_id INTEGER,
            title_type TEXT NOT NULL,
            division TEXT NOT NULL,
            prestige_tier INTEGER NOT NULL,
            gender TEXT NOT NULL,
            show_id INTEGER,
            is_active BOOLEAN NOT NULL DEFAULT TRUE,
            is_user_created BOOLEAN DEFAULT FALSE,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )
    "#).execute(conn).expect("Failed to create titles table");

    diesel::sql_query(r#"
        CREATE TABLE title_holders (
            id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
            title_id INTEGER NOT NULL,
            wrestler_id INTEGER NOT NULL,
            held_since TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
            held_until TIMESTAMP NULL,
            event_name TEXT NULL,
            event_location TEXT NULL,
            change_method TEXT NULL,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )
    "#).execute(conn).expect("Failed to create title_holders table");

    diesel::sql_query(r#"
        CREATE TABLE show_rosters (
            id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
            show_id INTEGER NOT NULL,
            wrestler_id INTEGER NOT NULL,
            assigned_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            is_active BOOLEAN NOT NULL DEFAULT TRUE
        )
    "#).execute(conn).expect("Failed to create show_rosters table");
    
    // Migration 4: Create match system
    diesel::sql_query(r#"
        CREATE TABLE matches (
            id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
            show_id INTEGER NOT NULL,
            match_name TEXT NOT NULL,
            match_type TEXT NOT NULL,
            match_stipulation TEXT NULL,
            scheduled_date TIMESTAMP NULL,
            match_order INTEGER NULL,
            winner_id INTEGER NULL,
            is_title_match BOOLEAN NOT NULL DEFAULT FALSE,
            title_id INTEGER NULL,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )
    "#).execute(conn).expect("Failed to create matches table");

    diesel::sql_query(r#"
        CREATE TABLE match_participants (
            id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
            match_id INTEGER NOT NULL,
            wrestler_id INTEGER NOT NULL,
            team_number INTEGER NULL,
            entrance_order INTEGER NULL,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )
    "#).execute(conn).expect("Failed to create match_participants table");
    
    // Verify tables were created successfully
    println!("✓ All test database tables created successfully");
}

pub struct TestData {
    pub pool: Pool<ConnectionManager<SqliteConnection>>,
}

impl Default for TestData {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for TestData {
    fn drop(&mut self) {
        // Clean up test database when TestData is dropped
        let test_db_path = "./test_database.db";
        if std::path::Path::new(test_db_path).exists() {
            fs::remove_file(test_db_path).ok();
        }
    }
}

impl TestData {
    pub fn new() -> Self {
        Self {
            pool: setup_test_db(),
        }
    }
    
    /// Explicit cleanup method for test database
    pub fn cleanup_test_database(&self) {
        let test_db_path = "./test_database.db";
        if std::path::Path::new(test_db_path).exists() {
            fs::remove_file(test_db_path).ok();
        }
    }

    pub fn get_connection(
        &self,
    ) -> diesel::r2d2::PooledConnection<ConnectionManager<SqliteConnection>> {
        self.pool.get().expect("Failed to get connection from pool")
    }

    #[allow(dead_code)]
    pub fn cleanup_users(&self, user_username: &str) {
        use wwe_universe_manager_lib::schema::users::dsl::*;
        let mut conn = self.get_connection();
        diesel::delete(users.filter(username.eq(user_username)))
            .execute(&mut conn)
            .ok();
    }

    #[allow(dead_code)]
    pub fn cleanup_wrestlers(&self, wrestler_name: &str) {
        use wwe_universe_manager_lib::schema::wrestlers::dsl::*;
        let mut conn = self.get_connection();
        diesel::delete(wrestlers.filter(name.eq(wrestler_name)))
            .execute(&mut conn)
            .ok();
    }

    #[allow(dead_code)]
    pub fn cleanup_titles(&self, title_name: &str) {
        use wwe_universe_manager_lib::schema::titles::dsl::*;
        let mut conn = self.get_connection();
        diesel::delete(titles.filter(name.eq(title_name)))
            .execute(&mut conn)
            .ok();
    }

    #[allow(dead_code)]
    pub fn cleanup_shows(&self, show_name: &str) {
        use wwe_universe_manager_lib::schema::shows::dsl::*;
        let mut conn = self.get_connection();
        diesel::delete(shows.filter(name.eq(show_name)))
            .execute(&mut conn)
            .ok();
    }

    #[allow(dead_code)]
    pub fn cleanup_signature_moves(&self, target_wrestler_id: i32) {
        use wwe_universe_manager_lib::schema::signature_moves::dsl::*;
        let mut conn = self.get_connection();
        diesel::delete(signature_moves.filter(wrestler_id.eq(target_wrestler_id)))
            .execute(&mut conn)
            .ok();
    }
}

#[allow(dead_code)]
pub fn create_test_user() -> NewUser {
    NewUser {
        username: "test_user".to_string(),
        password: "test_password".to_string(),
    }
}

#[allow(dead_code)]
pub fn create_test_wrestler() -> NewWrestler {
    NewWrestler {
        name: "Test Wrestler".to_string(),
        gender: "Male".to_string(),
        wins: 0,
        losses: 0,
        is_user_created: Some(false),
    }
}

#[allow(dead_code)]
pub fn create_test_enhanced_wrestler() -> NewEnhancedWrestler {
    NewEnhancedWrestler {
        name: "Enhanced Test Wrestler".to_string(),
        gender: "Male".to_string(),
        wins: 5,
        losses: 2,
        real_name: Some("John Test Doe".to_string()),
        nickname: Some("The Tester".to_string()),
        height: Some("6'2\"".to_string()),
        weight: Some("220 lbs".to_string()),
        debut_year: Some(2020),
        // promotion field removed - wrestlers are now global entities
        strength: Some(7),
        speed: Some(6),
        agility: Some(8),
        stamina: Some(7),
        charisma: Some(9),
        technique: Some(8),
        biography: Some("A test wrestler for testing enhanced features.".to_string()),
        is_user_created: Some(false),
    }
}

#[allow(dead_code)]
pub fn create_test_signature_move() -> NewSignatureMove {
    NewSignatureMove {
        wrestler_id: 1, // This will need to be set to actual wrestler ID in tests
        move_name: "Test Finisher".to_string(),
        move_type: "primary".to_string(),
    }
}

#[allow(dead_code)]
pub fn create_test_title() -> NewTitle {
    NewTitle {
        name: "Test Title".to_string(),
        current_holder_id: None,
        title_type: "Singles".to_string(),
        division: "World".to_string(),
        prestige_tier: 1,
        gender: "Mixed".to_string(),
        show_id: None,
        is_active: true,
        is_user_created: Some(false),
    }
}

#[allow(dead_code)]
pub fn create_test_show() -> NewShow {
    NewShow {
        name: "Test Show".to_string(),
        description: "A test show for testing purposes".to_string(),
    }
}
