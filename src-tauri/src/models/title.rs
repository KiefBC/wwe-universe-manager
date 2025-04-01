// models/title.rs
use crate::models::Wrestler;
use crate::schema::titles;
use diesel::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Debug, Queryable, Selectable, Identifiable, Associations, Serialize, Deserialize)]
#[diesel(table_name = titles)]
#[diesel(belongs_to(Wrestler, foreign_key = current_holder_id))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Title {
    pub id: i32,
    pub name: String,
    pub current_holder_id: Option<i32>,
}

// Struct for Diesel insertion (uses owned String, no lifetime)
#[derive(Insertable)]
#[diesel(table_name = titles)]
pub struct NewTitle {
    pub name: String,
}

// Struct for Tauri command argument
#[derive(Deserialize)]
pub struct TitleData {
    pub name: String,
    pub current_holder_id: Option<i32>,
}
