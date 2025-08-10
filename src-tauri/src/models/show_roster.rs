use crate::models::{Show, Wrestler};
use crate::schema::show_rosters;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Queryable, Selectable, Identifiable, Associations, Serialize, Deserialize)]
#[diesel(table_name = show_rosters)]
#[diesel(belongs_to(Show, foreign_key = show_id))]
#[diesel(belongs_to(Wrestler, foreign_key = wrestler_id))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct ShowRoster {
    pub id: i32,
    pub show_id: i32,
    pub wrestler_id: i32,
    pub assigned_at: Option<NaiveDateTime>,
    pub is_active: bool,
}

#[derive(Insertable)]
#[diesel(table_name = show_rosters)]
pub struct NewShowRoster {
    pub show_id: i32,
    pub wrestler_id: i32,
    pub assigned_at: Option<NaiveDateTime>,
    pub is_active: bool,
}

#[derive(Deserialize)]
pub struct ShowRosterData {
    pub show_id: i32,
    pub wrestler_id: i32,
}