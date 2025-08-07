//! Match models and data structures
//! 
//! This module contains all match-related database models and data transfer objects.
//! Matches represent wrestling contests that take place on shows.

use crate::models::{Show, Title, Wrestler};
use crate::schema::matches;
use chrono::{NaiveDate, NaiveDateTime};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

/// Main match model representing a wrestling match
/// 
/// Matches are wrestling contests that take place on shows. They can be
/// singles, tag team, or multi-person matches with various stipulations.
/// Title matches are linked to specific championships.
#[derive(Debug, Queryable, Selectable, Identifiable, Associations, Serialize, Deserialize)]
#[diesel(table_name = matches)]
#[diesel(belongs_to(Show, foreign_key = show_id))]
#[diesel(belongs_to(Wrestler, foreign_key = winner_id))]
#[diesel(belongs_to(Title, foreign_key = title_id))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Match {
    /// Unique identifier
    pub id: i32,
    /// ID of the show this match belongs to
    pub show_id: i32,
    /// Optional match name (e.g., "World Championship Match")
    pub match_name: Option<String>,
    /// Type of match ("Singles", "Tag Team", "Triple Threat", etc.)
    pub match_type: String,
    /// Optional stipulation ("No Disqualification", "Cage Match", etc.)
    pub match_stipulation: Option<String>,
    /// Date when the match is scheduled
    pub scheduled_date: Option<NaiveDate>,
    /// Order of the match on the card (1 = opening match)
    pub match_order: Option<i32>,
    /// ID of the winning wrestler (None if match hasn't concluded)
    pub winner_id: Option<i32>,
    /// Whether this is a championship match
    pub is_title_match: bool,
    /// ID of the title at stake (if is_title_match is true)
    pub title_id: Option<i32>,
    /// Timestamp when the match was created
    pub created_at: Option<NaiveDateTime>,
    /// Timestamp when the match was last updated
    pub updated_at: Option<NaiveDateTime>,
}

/// Model for creating a new match
/// 
/// Used when inserting a new match into the database.
/// Winner is typically set to None initially and updated after the match.
#[derive(Insertable)]
#[diesel(table_name = matches)]
pub struct NewMatch {
    pub show_id: i32,
    pub match_name: Option<String>,
    pub match_type: String,
    pub match_stipulation: Option<String>,
    pub scheduled_date: Option<NaiveDate>,
    pub match_order: Option<i32>,
    pub winner_id: Option<i32>,
    pub is_title_match: bool,
    pub title_id: Option<i32>,
}

/// Data transfer object for match creation via API
/// 
/// Used by the frontend when creating a new match.
/// The scheduled_date string should be in "YYYY-MM-DD" format.
#[derive(Deserialize)]
pub struct MatchData {
    pub show_id: i32,
    pub match_name: Option<String>,
    pub match_type: String,
    pub match_stipulation: Option<String>,
    pub scheduled_date: Option<String>, // Will be parsed to NaiveDate
    pub match_order: Option<i32>,
    pub is_title_match: bool,
    pub title_id: Option<i32>,
}