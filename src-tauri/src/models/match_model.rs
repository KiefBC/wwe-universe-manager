use crate::models::{Show, Title, Wrestler};
use crate::schema::matches;
use chrono::{NaiveDate, NaiveDateTime};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Queryable, Selectable, Identifiable, Associations, Serialize, Deserialize)]
#[diesel(table_name = matches)]
#[diesel(belongs_to(Show, foreign_key = show_id))]
#[diesel(belongs_to(Wrestler, foreign_key = winner_id))]
#[diesel(belongs_to(Title, foreign_key = title_id))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Match {
    pub id: i32,
    pub show_id: i32,
    pub match_name: Option<String>,
    pub match_type: String,
    pub match_stipulation: Option<String>,
    pub scheduled_date: Option<NaiveDate>,
    pub match_order: Option<i32>,
    pub winner_id: Option<i32>,
    pub is_title_match: bool,
    pub title_id: Option<i32>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

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