use crate::schema::wrestlers;
use diesel::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Debug, Queryable, Selectable, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = wrestlers)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Wrestler {
    pub id: i32,
    pub name: String,
    pub gender: String,
    pub wins: i32,
    pub losses: i32,
}

#[derive(Insertable)]
#[diesel(table_name = wrestlers)]
pub struct NewWrestler {
    pub name: String,
    pub gender: String,
}

#[derive(Deserialize)]
pub struct WrestlerData {
    pub name: String,
    pub gender: String,
}
