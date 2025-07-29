// models/title.rs
use crate::models::{Show, Wrestler};
use crate::schema::titles;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Queryable, Selectable, Identifiable, Associations, Serialize, Deserialize)]
#[diesel(table_name = titles)]
#[diesel(belongs_to(Wrestler, foreign_key = current_holder_id))]
#[diesel(belongs_to(Show, foreign_key = show_id))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Title {
    pub id: i32,
    pub name: String,
    pub current_holder_id: Option<i32>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub title_type: String,
    pub division: String,
    pub prestige_tier: i32,
    pub gender: String,
    pub show_id: Option<i32>,
    pub is_active: bool,
    pub is_user_created: Option<bool>,
}

// Struct for Diesel insertion (uses owned String, no lifetime)
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

// Struct for Tauri command argument
#[derive(Deserialize)]
pub struct TitleData {
    pub name: String,
    pub current_holder_id: Option<i32>,
    pub title_type: String,
    pub division: String,
    pub gender: String,
    pub show_id: Option<i32>,
}
