use diesel::connection::SimpleConnection;
use diesel::{Connection, SqliteConnection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use dotenvy::dotenv;
use std::env;
use std::error::Error;
use std::path::PathBuf;
use tauri::{AppHandle, Manager};

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

pub struct Database {
    pub conn: SqliteConnection,
}

impl Database {
    pub fn new(app_handle: &AppHandle) -> Result<Self, String> {
        let db_url = get_database_url(app_handle.path().app_local_data_dir().unwrap());

        let mut conn = establish_connection(db_url);

        match conn.batch_execute("PRAGMA foreign_keys = ON;") {
            Ok(_) => match run_migrations(&mut conn) {
                Ok(_) => Ok(Database {
                    conn,
                }),
                Err(_) => Err("Erreur lors du chargement de la base de donnée".to_string())
            },
            Err(_) => Err("Erreur lors du chargement de la base de donnée".to_string())
        }
    }
}

pub fn get_database_url(app_local_data_dir: PathBuf) -> String {
    dotenv().ok();

    if cfg!(debug_assertions) {
        env::var("DATABASE_URL").expect("DATABASE_URL non défini")
    } else {
        let db_path = app_local_data_dir.join("db.sqlite");
        std::fs::create_dir_all(app_local_data_dir)
            .expect("Erreur lors de la création de la base de données");
        String::from(db_path.to_str().unwrap())
    }
}

fn establish_connection(db_url: String) -> SqliteConnection {
    SqliteConnection::establish(&db_url)
        .unwrap_or_else(|_| panic!("Connexion impossible à {}", db_url))
}

fn run_migrations(
    conn: &mut SqliteConnection,
) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    conn.run_pending_migrations(MIGRATIONS)?;

    Ok(())
}
