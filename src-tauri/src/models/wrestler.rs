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
}

#[derive(Insertable)]
#[diesel(table_name = wrestlers)]
pub struct NewWrestler {
    pub name: String,
    pub gender: String,
    pub wins: i32,
    pub losses: i32,
}

#[derive(Deserialize)]
pub struct WrestlerData {
    pub name: String,
    pub gender: Gender,
}
