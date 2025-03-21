use crate::schema::wrestlers;
use diesel::prelude::*;

#[derive(Debug, Queryable, Selectable, Identifiable)]
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
pub struct NewWrestler<'a> {
    pub name: &'a str,
    pub gender: &'a str,
}
