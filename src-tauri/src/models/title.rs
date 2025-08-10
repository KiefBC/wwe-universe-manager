//\! Championship title models and data structures
//\! 
//\! This module contains all title-related database models and data transfer objects.
//\! Titles are global entities that can be assigned to specific shows or remain
//\! cross-brand.

use crate::models::{Show, Wrestler};
use crate::schema::titles;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

/// Main championship title model
/// 
/// Titles represent championships that wrestlers compete for. They are global
/// entities that can be assigned to specific shows or remain cross-brand.
/// Each title has a prestige tier, gender restrictions, and tracking for
/// current and historical holders.
/// 
/// IMPORTANT: Field order must exactly match database schema column order\!
#[derive(Debug, Queryable, Selectable, Identifiable, Associations, Serialize, Deserialize)]
#[diesel(table_name = titles)]
#[diesel(belongs_to(Wrestler, foreign_key = current_holder_id))]
#[diesel(belongs_to(Show, foreign_key = show_id))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Title {
    /// Unique identifier
    pub id: i32,
    /// Title name (e.g., "WWE Championship", "Women's World Championship")
    pub name: String,
    /// ID of the current champion (None if vacant)
    pub current_holder_id: Option<i32>,
    /// Type of title ("Singles", "Tag Team", etc.)
    pub title_type: String,
    /// Division/category ("World", "Intercontinental", "Women's World", etc.)
    pub division: String,
    /// Prestige tier (1=World, 2=Secondary, 3=Tag, 4=Specialty)
    pub prestige_tier: i32,
    /// Gender restriction ("Male", "Female", "Mixed")
    pub gender: String,
    /// ID of assigned show (None for cross-brand titles)
    pub show_id: Option<i32>,
    /// Whether the title is currently active
    pub is_active: bool,
    /// Whether this title was created by a user (vs system-created)
    pub is_user_created: Option<bool>,
    /// Timestamp when the title was created
    pub created_at: Option<NaiveDateTime>,
    /// Timestamp when the title was last updated
    pub updated_at: Option<NaiveDateTime>,
}

/// Model for creating a new championship title
/// 
/// Used when inserting a new title into the database.
/// Prestige tier is typically calculated based on division.
#[derive(Insertable)]
#[diesel(table_name = titles)]
pub struct NewTitle {
    pub name: String,
    pub current_holder_id: Option<i32>,
    pub title_type: String,
    pub division: String,
    pub prestige_tier: i32,
    pub gender: String,
    pub show_id: Option<i32>,
    pub is_active: bool,
    pub is_user_created: Option<bool>,
}

/// Data transfer object for title creation via API
/// 
/// Used by the frontend when creating a new championship title.
/// Prestige tier is calculated automatically based on division.
#[derive(Deserialize)]
pub struct TitleData {
    pub name: String,
    pub current_holder_id: Option<i32>,
    pub title_type: String,
    pub division: String,
    pub gender: String,
    pub show_id: Option<i32>,
}

