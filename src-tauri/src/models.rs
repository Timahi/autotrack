use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Identifiable, Serialize)]
#[diesel(table_name = crate::schema::profiles)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Profile {
    pub id: i32,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = crate::schema::profiles)]
pub struct NewProfile {
    pub name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(AsChangeset, Deserialize)]
#[diesel(table_name = crate::schema::profiles)]
pub struct EditProfile {
    pub name: String,
    pub updated_at: NaiveDateTime,
}

#[derive(Queryable, Selectable, Identifiable, Serialize, Associations)]
#[diesel(table_name = crate::schema::vehicles)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(belongs_to(Profile))]
pub struct Vehicle {
    pub id: i32,
    pub brand: String,
    pub model: String,
    pub odometer: i32,
    pub odometer_updated_at: NaiveDateTime,
    pub registration: String,
    pub registration_year: i32,
    pub serial_number: Option<String>,
    pub description: Option<String>,
    pub profile_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = crate::schema::vehicles)]
pub struct NewVehicle {
    pub brand: String,
    pub model: String,
    pub odometer: i32,
    pub odometer_updated_at: NaiveDateTime,
    pub registration: String,
    pub registration_year: i32,
    pub serial_number: Option<String>,
    pub description: Option<String>,
    pub profile_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(AsChangeset, Deserialize)]
#[diesel(table_name = crate::schema::vehicles)]
pub struct EditVehicle {
    pub brand: String,
    pub model: String,
    pub odometer: i32,
    pub odometer_updated_at: NaiveDateTime,
    pub registration: String,
    pub registration_year: i32,
    pub serial_number: Option<String>,
    pub description: Option<String>,
    pub profile_id: i32,
    pub updated_at: NaiveDateTime,
}