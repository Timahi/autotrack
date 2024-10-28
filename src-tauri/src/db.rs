use diesel::{Connection, SqliteConnection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationError, MigrationHarness};
use dotenvy::dotenv;
use std::env;
use std::error::Error;
use std::path::PathBuf;
use std::sync::Mutex;
use tauri::path::PathResolver;
use tauri::{AppHandle, Manager};

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

pub struct Database {
    pub conn: Mutex<SqliteConnection>,
}

impl Database {
    pub fn new(app_handle: &AppHandle) -> Self {
        let db_url = get_database_url(app_handle.path().app_local_data_dir().unwrap());

        let mut conn = SqliteConnection::establish(&db_url)
            .expect("Impossible d'établir la connexion à la base de données");

        if let Err(err) = run_migrations(&mut conn) {
            panic!("Erreur lors de l'application des migrations: {}", err);
        }

        Database {
            conn: Mutex::new(conn),
        }
    }
}

fn get_database_url(app_local_data_dir: PathBuf) -> String {
    dotenv().ok();

    if cfg!(debug_assertions) {
        env::var("DATABASE_URL").expect("DATABASE_URL non défini")
    } else {
        let db_path = app_local_data_dir.join("database.sqlite");
        std::fs::create_dir_all(app_local_data_dir).expect("Erreur lors de la création de la base de données");
        String::from(db_path.to_str().unwrap())
    }
}

fn establish_connection(db_url: String) -> SqliteConnection {
    SqliteConnection::establish(&db_url).unwrap_or_else(|_| panic!("Connexion impossible à {}", db_url))
}

fn run_migrations(conn: &mut SqliteConnection) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    conn.run_pending_migrations(MIGRATIONS)?;

    Ok(())
}