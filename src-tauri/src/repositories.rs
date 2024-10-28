use crate::models::*;
use crate::schema::profiles::dsl::*;
use chrono::Utc;
use diesel::prelude::*;
use diesel::result::{DatabaseErrorKind, Error};

pub fn get_profiles(conn: &mut SqliteConnection) -> Result<Vec<Profile>, String> {
    profiles
        .select(Profile::as_select())
        .get_results(conn)
        .map_err(|_| "Échec lors de la récupération des profils".to_string())
}

pub fn get_profile_by_id(conn: &mut SqliteConnection, profile_id: i32) -> Result<Profile, String> {
    profiles
        .find(profile_id)
        .select(Profile::as_select())
        .get_result(conn)
        .map_err(|_| "Échec lors de la récupération du profil".to_string())
}

pub fn create_profile(
    conn: &mut SqliteConnection,
    new_profile: NewProfile,
) -> Result<Profile, String> {
    match diesel::insert_into(profiles)
        .values(&new_profile)
        .returning(Profile::as_returning())
        .get_result(conn)
    {
        Ok(profile) => Ok(profile),
        Err(Error::DatabaseError(DatabaseErrorKind::UniqueViolation, _)) => {
            Err("Un profil avec ce nom existe déjà".to_string())
        }
        Err(_) => Err("Échec lors de la création du profil".to_string()),
    }
}

pub fn update_profile(
    conn: &mut SqliteConnection,
    profile_id: i32,
    new_profile: NewProfile,
) -> Result<Profile, String> {
    diesel::update(profiles.find(profile_id))
        .set((
            name.eq(new_profile.name),
            updated_at.eq(Utc::now().naive_utc()),
        ))
        .returning(Profile::as_returning())
        .get_result(conn)
        .map_err(|_| "Échec lors de la mise à jour du profil".to_string())
}

pub fn delete_profile(conn: &mut SqliteConnection, profile_id: i32) -> Result<usize, String> {
    diesel::delete(profiles.find(profile_id))
        .execute(conn)
        .map_err(|_| "Échec lors de la suppression du profil".to_string())
}
