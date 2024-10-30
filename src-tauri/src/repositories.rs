use crate::models::*;
use diesel::prelude::*;
use diesel::result::{DatabaseErrorKind, Error};

pub fn get_profiles(conn: &mut SqliteConnection) -> Result<Vec<Profile>, String> {
    use crate::schema::profiles::dsl::*;

    match profiles
        .get_results(conn)
    {
        Ok(p) => Ok(p),
        Err(_) => Err("Échec lors de la récupération des profils".to_string())
    }
}

pub fn get_profile_by_id(conn: &mut SqliteConnection, profile_id: i32) -> Result<Profile, String> {
    use crate::schema::profiles::dsl::*;

    match profiles
        .find(profile_id)
        .get_result(conn)
    {
        Ok(p) => Ok(p),
        Err(_) => Err("Échec lors de la récupération des profils".to_string())
    }
}

pub fn create_profile(
    conn: &mut SqliteConnection,
    new_profile: NewProfile,
) -> Result<Profile, String> {
    use crate::schema::profiles::dsl::*;

    match diesel::insert_into(profiles)
        .values(&new_profile)
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
    edit_profile: EditProfile,
) -> Result<Profile, String> {
    use crate::schema::profiles::dsl::*;

    match diesel::update(profiles.find(profile_id))
        .set(&edit_profile)
        .get_result(conn)
    {
        Ok(profile) => Ok(profile),
        Err(Error::DatabaseError(DatabaseErrorKind::UniqueViolation, _)) => {
            Err("Un profil avec ce nom existe déjà".to_string())
        }
        Err(_) => Err("Échec lors de la mise à jour du profil".to_string()),
    }
}

pub fn delete_profile(conn: &mut SqliteConnection, profile_id: i32) -> Result<(), String> {
    use crate::schema::profiles::dsl::*;

    match diesel::delete(profiles.find(profile_id))
        .execute(conn) {
        Ok(_) => Ok(()),
        Err(_) => Err("Échec lors de la suppression du profil".to_string())
    }
}

pub fn get_vehicles(conn: &mut SqliteConnection, _profile_id: i32) -> Result<Vec<Vehicle>, String> {
    use crate::schema::vehicles::dsl::*;

    match vehicles
        .filter(profile_id.eq(_profile_id))
        .get_results(conn)
    {
        Ok(v) => Ok(v),
        Err(_) => Err("Échec lors de la récupération des véhicules".to_string())
    }
}

pub fn get_vehicle_by_id(conn: &mut SqliteConnection, _profile_id: i32, vehicle_id: i32) -> Result<Vehicle, String> {
    use crate::schema::vehicles::dsl::*;

    match vehicles
        .find(vehicle_id)
        .filter(profile_id.eq(_profile_id))
        .get_result(conn)
    {
        Ok(v) => Ok(v),
        Err(_) => Err("Échec lors de la récupération du véhicule".to_string())
    }
}

pub fn create_vehicle(
    conn: &mut SqliteConnection,
    new_vehicle: NewVehicle,
) -> Result<Vehicle, String> {
    use crate::schema::vehicles::dsl::*;

    match diesel::insert_into(vehicles)
        .values(&new_vehicle)
        .returning(Vehicle::as_returning())
        .get_result(conn)
    {
        Ok(v) => Ok(v),
        Err(_) => Err("Échec lors de la création du véhicule".to_string()),
    }
}

pub fn update_vehicle(conn: &mut SqliteConnection, vehicle_id: i32, edit_vehicle: EditVehicle) -> Result<Vehicle, String> {
    use crate::schema::vehicles::dsl::*;

    match diesel::update(vehicles.find(vehicle_id))
        .set(&edit_vehicle)
        .get_result(conn)
    {
        Ok(v) => Ok(v),
        Err(_) => Err("Échec lors de la mise à jour du véhicule".to_string())
    }
}

pub fn delete_vehicle(conn: &mut SqliteConnection, vehicle_id: i32) -> Result<(), String> {
    use crate::schema::vehicles::dsl::*;

    match diesel::delete(vehicles.find(vehicle_id))
        .execute(conn)
    {
        Ok(_) => Ok(()),
        Err(_) => Err("Échec lors de la suppression du véhicule".to_string())
    }
}