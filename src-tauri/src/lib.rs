use crate::db::{get_database_url, Database};
use crate::models::{EditProfile, EditVehicle, NewProfile, NewVehicle, Profile, Vehicle};
use crate::repositories::*;
use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;
use tauri::{AppHandle, Manager, State};

mod schema;
mod db;
mod models;
mod repositories;

struct AppState {
    db: Database,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            match Database::new(&app.handle()) {
                Ok(db) => {
                    app.manage(Mutex::new(AppState {
                        db
                    }));
                }
                Err(err) => panic!("{}", err)
            }


            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            import_database_to_path_command, export_database_to_path_command,
            get_profiles_command, get_profile_by_id_command, create_profile_command, update_profile_command, delete_profile_command,
            get_vehicles_command, get_vehicle_by_id_command, create_vehicle_command, update_vehicle_command, delete_vehicle_command
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn import_database_to_path_command(app_handle: AppHandle, path: String) -> Result<(), String> {
    let db_path = PathBuf::from(get_database_url(app_handle.path().app_local_data_dir().unwrap()));
    let backup_db_path = app_handle.path().temp_dir().unwrap().join("backup_autotrack.db");

    if let Err(_) = fs::copy(&db_path, &backup_db_path) {
        return Err("Échec lors de l'importation des données".to_string());
    }

    match fs::copy(&path, &db_path) {
        Ok(_) => {
            match Database::new(&app_handle) {
                Ok(db) => {
                    app_handle.manage(Mutex::new(AppState { db }));
                    Ok(())
                }
                Err(_) => {
                    if let Err(_) = fs::copy(&backup_db_path, &db_path) {
                        return Err("Erreur lors de la restauration de l'ancienne base de données".to_string());
                    }
                    Err("Erreur lors de l'importation de la base de données".to_string())
                }
            }
        }
        Err(_) => Err("Échec lors de l'importation des données".to_string()),
    }
}


#[tauri::command]
fn export_database_to_path_command(app_handle: AppHandle, path: String) -> Result<(),
    String> {
    let db_path = get_database_url(app_handle.path().app_local_data_dir().unwrap());

    match fs::copy(PathBuf::from(&db_path), PathBuf::from(path.to_string())) {
        Ok(_) => Ok(()),
        Err(e) => {
            eprintln!("Erreur lors de la copie de la base de données : {:?}", e);
            Err("Échec lors de l'exportation des données.".to_string())
        }
    }
}

#[tauri::command]
fn get_profiles_command(app_state: State<'_, Mutex<AppState>>) -> Result<Vec<Profile>, String> {
    let mut state = app_state.lock().unwrap();
    let conn = &mut state.db.conn;

    get_profiles(conn)
}

#[tauri::command]
fn get_profile_by_id_command(app_state: State<'_, Mutex<AppState>>, profile_id: i32) -> Result<Profile, String> {
    let mut state = app_state.lock().unwrap();
    let conn = &mut state.db.conn;

    get_profile_by_id(conn, profile_id)
}

#[tauri::command]
fn create_profile_command(app_state: State<'_, Mutex<AppState>>, new_profile: NewProfile) -> Result<Profile,
    String> {
    let mut state = app_state.lock().unwrap();
    let conn = &mut state.db.conn;

    create_profile(conn, new_profile)
}

#[tauri::command]
fn update_profile_command(app_state: State<'_, Mutex<AppState>>, profile_id: i32, edit_profile: EditProfile) -> Result<Profile,
    String> {
    let mut state = app_state.lock().unwrap();
    let conn = &mut state.db.conn;

    update_profile(conn, profile_id, edit_profile)
}

#[tauri::command]
fn delete_profile_command(app_state: State<'_, Mutex<AppState>>, profile_id: i32) -> Result<(),
    String> {
    let mut state = app_state.lock().unwrap();
    let conn = &mut state.db.conn;

    delete_profile(conn, profile_id)
}

#[tauri::command]
fn get_vehicles_command(app_state: State<'_, Mutex<AppState>>, profile_id: i32) -> Result<Vec<Vehicle>, String> {
    let mut state = app_state.lock().unwrap();
    let conn = &mut state.db.conn;

    get_vehicles(conn, profile_id)
}

#[tauri::command]
fn get_vehicle_by_id_command(app_state: State<'_, Mutex<AppState>>, vehicle_id: i32) -> Result<Vehicle, String> {
    let mut state = app_state.lock().unwrap();
    let conn = &mut state.db.conn;

    get_vehicle_by_id(conn, vehicle_id)
}

#[tauri::command]
fn create_vehicle_command(app_state: State<'_, Mutex<AppState>>, new_vehicle: NewVehicle) -> Result<Vehicle,
    String> {
    let mut state = app_state.lock().unwrap();
    let conn = &mut state.db.conn;

    create_vehicle(conn, new_vehicle)
}

#[tauri::command]
fn update_vehicle_command(app_state: State<'_, Mutex<AppState>>, vehicle_id: i32, edit_vehicle: EditVehicle) -> Result<Vehicle,
    String> {
    let mut state = app_state.lock().unwrap();
    let conn = &mut state.db.conn;

    update_vehicle(conn, vehicle_id, edit_vehicle)
}

#[tauri::command]
fn delete_vehicle_command(app_state: State<'_, Mutex<AppState>>, vehicle_id: i32) -> Result<(),
    String> {
    let mut state = app_state.lock().unwrap();
    let conn = &mut state.db.conn;

    delete_vehicle(conn, vehicle_id)
}
