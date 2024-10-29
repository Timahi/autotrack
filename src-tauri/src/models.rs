use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::Serialize;

#[derive(Queryable, Selectable, Identifiable, Serialize)]
#[diesel(table_name = crate::schema::profiles)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Profile {
    pub id: i32,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::profiles)]
pub struct NewProfile<'a> {
    pub name: &'a str,
    pub created_at: &'a NaiveDateTime,
    pub updated_at: &'a NaiveDateTime,
}

#[derive(AsChangeset)]
#[diesel(table_name = crate::schema::profiles)]
pub struct EditProfile<'a> {
    pub name: &'a str,
    pub updated_at: &'a NaiveDateTime,
}
