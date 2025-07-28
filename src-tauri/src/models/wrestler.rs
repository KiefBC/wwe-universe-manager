use crate::schema::wrestlers;
use crate::types::Gender;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Queryable, Selectable, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = wrestlers)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Wrestler {
    pub id: i32,
    pub name: String,
    pub gender: String,
    pub wins: i32,
    pub losses: i32,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    // Enhanced details
    pub real_name: Option<String>,
    pub nickname: Option<String>,
    pub height: Option<String>,
    pub weight: Option<String>,
    pub debut_year: Option<i32>,
    pub promotion: Option<String>,
    // Power ratings (1-10)
    pub strength: Option<i32>,
    pub speed: Option<i32>,
    pub agility: Option<i32>,
    pub stamina: Option<i32>,
    pub charisma: Option<i32>,
    pub technique: Option<i32>,
    // Content
    pub biography: Option<String>,
    pub is_user_created: Option<bool>,
}

#[derive(Insertable)]
#[diesel(table_name = wrestlers)]
pub struct NewWrestler {
    pub name: String,
    pub gender: String,
    pub wins: i32,
    pub losses: i32,
    pub is_user_created: Option<bool>,
}

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
    pub promotion: Option<String>,
    pub strength: Option<i32>,
    pub speed: Option<i32>,
    pub agility: Option<i32>,
    pub stamina: Option<i32>,
    pub charisma: Option<i32>,
    pub technique: Option<i32>,
    pub biography: Option<String>,
    pub is_user_created: Option<bool>,
}

#[derive(Deserialize)]
pub struct WrestlerData {
    pub name: String,
    pub gender: Gender,
}

#[derive(Deserialize)]
pub struct EnhancedWrestlerData {
    pub name: String,
    pub gender: Gender,
    pub real_name: Option<String>,
    pub nickname: Option<String>,
    pub height: Option<String>,
    pub weight: Option<String>,
    pub debut_year: Option<i32>,
    pub promotion: Option<String>,
    pub strength: Option<i32>,
    pub speed: Option<i32>,
    pub agility: Option<i32>,
    pub stamina: Option<i32>,
    pub charisma: Option<i32>,
    pub technique: Option<i32>,
    pub biography: Option<String>,
}
