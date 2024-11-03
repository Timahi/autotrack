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
    pub name: Option<String>,
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
    pub brand: Option<String>,
    pub model: Option<String>,
    pub odometer: Option<i32>,
    pub odometer_updated_at: Option<NaiveDateTime>,
    pub registration: Option<String>,
    pub registration_year: Option<i32>,
    pub serial_number: Option<String>,
    pub description: Option<String>,
    pub profile_id: Option<i32>,
    pub updated_at: NaiveDateTime,
}

#[derive(Queryable, Selectable, Identifiable, Serialize)]
#[diesel(table_name = crate::schema::inspections)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Inspection {
    pub id: i32,
    pub vehicle_id: i32,
    pub result: i32,
    pub performed_at: NaiveDateTime,
    pub next_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = crate::schema::inspections)]
pub struct NewInspection {
    pub vehicle_id: i32,
    pub result: i32,
    pub performed_at: NaiveDateTime,
    pub next_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(AsChangeset, Deserialize)]
#[diesel(table_name = crate::schema::inspections)]
pub struct EditInspection {
    pub vehicle_id: Option<i32>,
    pub result: Option<i32>,
    pub performed_at: Option<NaiveDateTime>,
    pub next_at: Option<NaiveDateTime>,
    pub updated_at: NaiveDateTime,
}

#[derive(Queryable, Selectable, Identifiable, Serialize)]
#[diesel(table_name = crate::schema::maintenance)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Maintenance {
    pub id: i32,
    pub vehicle_id: i32,
    #[serde(rename = "type")]
    pub maintenance_type: String,
    pub description: Option<String>,
    pub odometer: i32,
    pub performed_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = crate::schema::maintenance)]
pub struct NewMaintenance {
    pub vehicle_id: i32,
    pub maintenance_type: String,
    pub description: Option<String>,
    pub odometer: i32,
    pub performed_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(AsChangeset, Deserialize)]
#[diesel(table_name = crate::schema::maintenance)]
pub struct EditMaintenance {
    pub vehicle_id: Option<i32>,
    pub maintenance_type: Option<String>,
    pub description: Option<String>,
    pub odometer: Option<i32>,
    pub performed_at: Option<NaiveDateTime>,
    pub updated_at: NaiveDateTime,
}
