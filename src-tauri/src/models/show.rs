//! Show models and data structures
//! 
//! This module contains all show-related database models and data transfer objects.
//! Shows represent wrestling programs (e.g., Monday Night RAW, SmackDown) that belong
//! to specific promotions.

use crate::schema::shows;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

/// Main show model representing a wrestling show/program
/// 
/// Shows are promotion-specific programs where wrestling events take place.
/// Each show can have its own roster of wrestlers, championship titles,
/// and scheduled matches.
#[derive(Debug, Queryable, Selectable, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = shows)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Show {
    /// Unique identifier
    pub id: i32,
    /// Show name (e.g., "Monday Night RAW", "Friday Night SmackDown")
    pub name: String,
    /// Description of the show and its format
    pub description: String,
    /// Timestamp when the show was created
    pub created_at: Option<NaiveDateTime>,
    /// Timestamp when the show was last updated
    pub updated_at: Option<NaiveDateTime>,
}

/// Model for creating a new show
/// 
/// Used when inserting a new show into the database
#[derive(Insertable)]
#[diesel(table_name = shows)]
pub struct NewShow {
    pub name: String,
    pub description: String,
}

/// Data transfer object for show creation via API
/// 
/// Used by the frontend when creating a new show
#[derive(Deserialize)]
pub struct ShowData {
    pub name: String,
    pub description: String,
}
