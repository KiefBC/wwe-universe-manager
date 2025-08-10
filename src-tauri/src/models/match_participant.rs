use crate::models::{Match, Wrestler};
use crate::schema::match_participants;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Queryable, Selectable, Identifiable, Associations, Serialize, Deserialize)]
#[diesel(table_name = match_participants)]
#[diesel(belongs_to(Match, foreign_key = match_id))]
#[diesel(belongs_to(Wrestler, foreign_key = wrestler_id))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct MatchParticipant {
    pub id: i32,
    pub match_id: i32,
    pub wrestler_id: i32,
    pub team_number: Option<i32>,
    pub entrance_order: Option<i32>,
}

#[derive(Insertable)]
#[diesel(table_name = match_participants)]
pub struct NewMatchParticipant {
    pub match_id: i32,
    pub wrestler_id: i32,
    pub team_number: Option<i32>,
    pub entrance_order: Option<i32>,
}

#[derive(Deserialize)]
pub struct MatchParticipantData {
    pub match_id: i32,
    pub wrestler_id: i32,
    pub team_number: Option<i32>,
    pub entrance_order: Option<i32>,
}