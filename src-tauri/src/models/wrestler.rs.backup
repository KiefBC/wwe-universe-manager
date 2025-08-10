//! Wrestler models and data structures
//! 
//! This module contains all wrestler-related database models and data transfer objects.
//! Wrestlers are global entities that can be assigned to multiple shows.

use crate::schema::wrestlers;
use crate::types::Gender;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

/// Main wrestler model representing a complete wrestler record from the database
/// 
/// Wrestlers are global entities in the system and can be assigned to multiple shows
/// through the show_rosters table. Each wrestler has basic stats, physical attributes,
/// power ratings, and biographical information.
#[derive(Debug, Queryable, Selectable, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = wrestlers)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Wrestler {
    /// Unique identifier
    pub id: i32,
    /// Ring name used for performances
    pub name: String,
    /// Gender (stored as string: "Male", "Female", or other)
    pub gender: String,
    /// Total number of wins
    pub wins: i32,
    /// Total number of losses
    pub losses: i32,
    /// Timestamp when the wrestler was created
    pub created_at: Option<NaiveDateTime>,
    /// Timestamp when the wrestler was last updated
    pub updated_at: Option<NaiveDateTime>,
    // Enhanced details
    /// Real name of the wrestler
    pub real_name: Option<String>,
    /// Nickname or catchphrase (e.g., "The People's Champion")
    pub nickname: Option<String>,
    /// Height in format like "6'5\""
    pub height: Option<String>,
    /// Weight in format like "260 lbs"
    pub weight: Option<String>,
    /// Year of wrestling debut
    pub debut_year: Option<i32>,
    // Power ratings (1-10)
    /// Physical strength rating (1-10)
    pub strength: Option<i32>,
    /// Speed and quickness rating (1-10)
    pub speed: Option<i32>,
    /// Agility and flexibility rating (1-10)
    pub agility: Option<i32>,
    /// Endurance and stamina rating (1-10)
    pub stamina: Option<i32>,
    /// Charisma and crowd connection rating (1-10)
    pub charisma: Option<i32>,
    /// Technical wrestling ability rating (1-10)
    pub technique: Option<i32>,
    // Content
    /// Biography text describing the wrestler's history and character
    pub biography: Option<String>,
    /// Whether this wrestler was created by a user (vs system-created)
    pub is_user_created: Option<bool>,
}

/// Model for creating a new wrestler with basic information
/// 
/// Used when creating wrestlers with minimal details. The system will
/// set wins and losses to 0 by default.
#[derive(Insertable)]
#[diesel(table_name = wrestlers)]
pub struct NewWrestler {
    pub name: String,
    pub gender: String,
    pub wins: i32,
    pub losses: i32,
    pub is_user_created: Option<bool>,
}

/// Model for creating a new wrestler with complete enhanced details
/// 
/// Used when creating wrestlers with full profiles including power ratings,
/// physical attributes, and biographical information.
#[derive(Insertable)]
#[diesel(table_name = wrestlers)]
pub struct NewEnhancedWrestler {
    pub name: String,
    pub gender: String,
    pub wins: i32,
    pub losses: i32,
    pub real_name: Option<String>,
    pub nickname: Option<String>,
    pub height: Option<String>,
    pub weight: Option<String>,
    pub debut_year: Option<i32>,
    pub strength: Option<i32>,
    pub speed: Option<i32>,
    pub agility: Option<i32>,
    pub stamina: Option<i32>,
    pub charisma: Option<i32>,
    pub technique: Option<i32>,
    pub biography: Option<String>,
    pub is_user_created: Option<bool>,
}

/// Data transfer object for basic wrestler creation via API
/// 
/// Used by the frontend when creating a wrestler with minimal information
#[derive(Deserialize)]
pub struct WrestlerData {
    pub name: String,
    pub gender: Gender,
}

/// Data transfer object for creating a wrestler with full details via API
/// 
/// Used by the frontend when creating a complete wrestler profile with
/// all attributes, power ratings, and biographical information
#[derive(Deserialize)]
pub struct EnhancedWrestlerData {
    pub name: String,
    pub gender: Gender,
    pub real_name: Option<String>,
    pub nickname: Option<String>,
    pub height: Option<String>,
    pub weight: Option<String>,
    pub debut_year: Option<i32>,
    pub strength: Option<i32>,
    pub speed: Option<i32>,
    pub agility: Option<i32>,
    pub stamina: Option<i32>,
    pub charisma: Option<i32>,
    pub technique: Option<i32>,
    pub biography: Option<String>,
}
