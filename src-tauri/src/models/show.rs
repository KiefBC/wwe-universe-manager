use crate::schema::shows;
use diesel::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Debug, Queryable, Selectable, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = shows)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Show {
    pub id: i32,
    pub name: String,
    pub description: String,
}

#[derive(Insertable)]
#[diesel(table_name = shows)]
pub struct NewShow {
    pub name: String,
    pub description: String,
}

#[derive(Deserialize)]
pub struct ShowData {
    pub name: String,
    pub description: String,
}