use crate::schema::shows;
use diesel::prelude::*;

#[derive(Debug, Queryable, Selectable, Identifiable)]
#[diesel(table_name = shows)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Show {
    pub id: i32,
    pub name: String,
    pub description: String,
}

#[derive(Insertable)]
#[diesel(table_name = shows)]
pub struct NewShow<'a> {
    pub name: &'a str,
    pub description: &'a str,
}