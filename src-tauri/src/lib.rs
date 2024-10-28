use crate::db::Database;
use crate::models::{NewProfile, Profile};
use crate::repositories::{create_profile, get_profiles};
use chrono::{NaiveDateTime, Utc};
use diesel::SqliteConnection;
use serde::Serialize;
use std::sync::{Arc, Mutex};
use tauri::{AppHandle, Manager, State};

mod schema;
mod db;
mod models;
mod repositories;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }

            let database = Database::new(&app.handle());
            app.manage(database);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![get_profiles_handler, create_profile_handler])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn get_profiles_handler(db: State<Database>) -> Result<Vec<Profile>, String> {
    let mut conn = db.conn.lock().map_err(|_| "Échec lors de la récupération de la base de données".to_string())?;

    get_profiles(&mut conn)
}

#[tauri::command]
fn create_profile_handler(db: State<Database>, name: String) -> Result<Profile, String> {
    let new_profile = NewProfile { name: &name, created_at: &Utc::now().naive_utc(), updated_at: &Utc::now().naive_utc() };

    let mut conn = db.conn.lock().map_err(|_| "Échec lors de la récupération de la base de données".to_string())?;

    create_profile(&mut conn, new_profile)
}
