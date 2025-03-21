// models/title.rs
use crate::models::Wrestler;
use crate::schema::titles;
use diesel::prelude::*;

#[derive(Debug, Queryable, Selectable, Identifiable, Associations)]
#[diesel(table_name = titles)]
#[diesel(belongs_to(Wrestler, foreign_key = current_holder_id))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Title {
    pub id: i32,
    pub name: String,
    pub current_holder_id: Option<i32>,
}

#[derive(Insertable)]
#[diesel(table_name = titles)]
pub struct NewTitle<'a> {
    pub name: &'a str,
}
