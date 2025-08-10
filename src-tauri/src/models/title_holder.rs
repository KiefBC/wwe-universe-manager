// models/title_holder.rs
use crate::models::{Title, Wrestler};
use crate::schema::title_holders;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Queryable, Selectable, Identifiable, Associations, Serialize, Deserialize)]
#[diesel(table_name = title_holders)]
#[diesel(belongs_to(Title, foreign_key = title_id))]
#[diesel(belongs_to(Wrestler, foreign_key = wrestler_id))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct TitleHolder {
    pub id: i32,
    pub title_id: i32,
    pub wrestler_id: i32,
    pub held_since: NaiveDateTime,
    pub held_until: Option<NaiveDateTime>,
    pub event_name: Option<String>,
    pub event_location: Option<String>,
    pub change_method: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

// Struct for Diesel insertion
#[derive(Insertable)]
#[diesel(table_name = title_holders)]
pub struct NewTitleHolder {
    pub title_id: i32,
    pub wrestler_id: i32,
    pub held_since: NaiveDateTime,
    pub event_name: Option<String>,
    pub event_location: Option<String>,
    pub change_method: Option<String>,
}

// Struct for Tauri command argument
#[derive(Deserialize)]
pub struct TitleHolderData {
    pub title_id: i32,
    pub wrestler_id: i32,
    pub event_name: Option<String>,
    pub event_location: Option<String>,
    pub change_method: Option<String>,
}

// Combined struct for displaying title with current holders
#[derive(Debug, Serialize, Deserialize)]
pub struct TitleWithHolders {
    pub title: Title,
    pub current_holders: Vec<TitleHolderInfo>,
    pub days_held: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TitleHolderInfo {
    pub holder: TitleHolder,
    pub wrestler_name: String,
    pub wrestler_gender: String,
}