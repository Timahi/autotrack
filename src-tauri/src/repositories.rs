use crate::models::*;
use diesel::prelude::*;
use diesel::result::{DatabaseErrorKind, Error};

pub fn get_profiles(conn: &mut SqliteConnection) -> Result<Vec<Profile>, String> {
    use crate::schema::profiles::dsl::*;

    match profiles.get_results(conn) {
        Ok(p) => Ok(p),
        Err(_) => Err("Échec lors de la récupération des profils".to_string()),
    }
}

pub fn get_profile_by_id(conn: &mut SqliteConnection, profile_id: i32) -> Result<Profile, String> {
    use crate::schema::profiles::dsl::*;

    match profiles.find(profile_id).get_result(conn) {
        Ok(p) => Ok(p),
        Err(_) => Err("Échec lors de la récupération des profils".to_string()),
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

    match diesel::delete(profiles.find(profile_id)).execute(conn) {
        Ok(_) => Ok(()),
        Err(_) => Err("Échec lors de la suppression du profil".to_string()),
    }
}

pub fn get_vehicles(conn: &mut SqliteConnection, _profile_id: i32) -> Result<Vec<Vehicle>, String> {
    use crate::schema::vehicles::dsl::*;

    match vehicles
        .filter(profile_id.eq(_profile_id))
        .get_results(conn)
    {
        Ok(v) => Ok(v),
        Err(_) => Err("Échec lors de la récupération des véhicules".to_string()),
    }
}

pub fn get_vehicle_by_id(conn: &mut SqliteConnection, vehicle_id: i32) -> Result<Vehicle, String> {
    use crate::schema::vehicles::dsl::*;

    match vehicles.find(vehicle_id).get_result(conn) {
        Ok(v) => Ok(v),
        Err(_) => Err("Échec lors de la récupération du véhicule".to_string()),
    }
}

pub fn create_vehicle(
    conn: &mut SqliteConnection,
    new_vehicle: NewVehicle,
) -> Result<Vehicle, String> {
    use crate::schema::vehicles::dsl::*;

    match diesel::insert_into(vehicles)
        .values(&new_vehicle)
        .get_result(conn)
    {
        Ok(v) => Ok(v),
        Err(_) => Err("Échec lors de la création du véhicule".to_string()),
    }
}

pub fn update_vehicle(
    conn: &mut SqliteConnection,
    vehicle_id: i32,
    edit_vehicle: EditVehicle,
) -> Result<Vehicle, String> {
    use crate::schema::vehicles::dsl::*;

    match diesel::update(vehicles.find(vehicle_id))
        .set(&edit_vehicle)
        .get_result(conn)
    {
        Ok(v) => Ok(v),
        Err(_) => Err("Échec lors de la mise à jour du véhicule".to_string()),
    }
}

pub fn delete_vehicle(conn: &mut SqliteConnection, vehicle_id: i32) -> Result<(), String> {
    use crate::schema::vehicles::dsl::*;

    match diesel::delete(vehicles.find(vehicle_id)).execute(conn) {
        Ok(_) => Ok(()),
        Err(_) => Err("Échec lors de la suppression du véhicule".to_string()),
    }
}

pub fn get_inspections(
    conn: &mut SqliteConnection,
    _vehicle_id: i32,
) -> Result<Vec<Inspection>, String> {
    use crate::schema::inspections::dsl::*;

    match inspections
        .filter(vehicle_id.eq(_vehicle_id))
        .get_results(conn)
    {
        Ok(v) => Ok(v),
        Err(_) => Err("Échec lors de la récupération des contrôles techniques".to_string()),
    }
}

pub fn get_inspection_by_id(
    conn: &mut SqliteConnection,
    inspection_id: i32,
) -> Result<Inspection, String> {
    use crate::schema::inspections::dsl::*;

    match inspections.find(inspection_id).get_result(conn) {
        Ok(v) => Ok(v),
        Err(_) => Err("Échec lors de la récupération du contrôle technique".to_string()),
    }
}

pub fn create_inspection(
    conn: &mut SqliteConnection,
    new_inspection: NewInspection,
) -> Result<Inspection, String> {
    use crate::schema::inspections::dsl::*;

    match diesel::insert_into(inspections)
        .values(&new_inspection)
        .get_result(conn)
    {
        Ok(v) => Ok(v),
        Err(_) => Err("Échec lors de la création du contrôle technique".to_string()),
    }
}

pub fn update_inspection(
    conn: &mut SqliteConnection,
    inspection_id: i32,
    edit_inspection: EditInspection,
) -> Result<Inspection, String> {
    use crate::schema::inspections::dsl::*;

    match diesel::update(inspections.find(inspection_id))
        .set(&edit_inspection)
        .get_result(conn)
    {
        Ok(v) => Ok(v),
        Err(_) => Err("Échec lors de la mise à jour du contrôle technique".to_string()),
    }
}

pub fn delete_inspection(conn: &mut SqliteConnection, inspection_id: i32) -> Result<(), String> {
    use crate::schema::inspections::dsl::*;

    match diesel::delete(inspections.find(inspection_id)).execute(conn) {
        Ok(_) => Ok(()),
        Err(_) => Err("Échec lors de la suppression du contrôle technique".to_string()),
    }
}

pub fn get_maintenance(
    conn: &mut SqliteConnection,
    _vehicle_id: i32,
) -> Result<Vec<Maintenance>, String> {
    use crate::schema::maintenance::dsl::*;

    match maintenance
        .filter(vehicle_id.eq(_vehicle_id))
        .get_results(conn)
    {
        Ok(v) => Ok(v),
        Err(_) => Err("Échec lors de la récupération des entretiens".to_string()),
    }
}

pub fn get_maintenance_by_id(
    conn: &mut SqliteConnection,
    maintenance_id: i32,
) -> Result<Maintenance, String> {
    use crate::schema::maintenance::dsl::*;

    match maintenance.find(maintenance_id).get_result(conn) {
        Ok(v) => Ok(v),
        Err(_) => Err("Échec lors de la récupération de l'entretien".to_string()),
    }
}

pub fn create_maintenance(
    conn: &mut SqliteConnection,
    new_maintenance: NewMaintenance,
) -> Result<Maintenance, String> {
    use crate::schema::maintenance::dsl::*;

    match diesel::insert_into(maintenance)
        .values(&new_maintenance)
        .get_result(conn)
    {
        Ok(v) => Ok(v),
        Err(_) => Err("Échec lors de la création de l'entretien".to_string()),
    }
}

pub fn update_maintenance(
    conn: &mut SqliteConnection,
    maintenance_id: i32,
    edit_maintenance: EditMaintenance,
) -> Result<Maintenance, String> {
    use crate::schema::maintenance::dsl::*;

    match diesel::update(maintenance.find(maintenance_id))
        .set(&edit_maintenance)
        .get_result(conn)
    {
        Ok(v) => Ok(v),
        Err(_) => Err("Échec lors de la mise à jour de l'entretien".to_string()),
    }
}

pub fn delete_maintenance(conn: &mut SqliteConnection, maintenance_id: i32) -> Result<(), String> {
    use crate::schema::maintenance::dsl::*;

    match diesel::delete(maintenance.find(maintenance_id)).execute(conn) {
        Ok(_) => Ok(()),
        Err(_) => Err("Échec lors de la suppression de la maintenance".to_string()),
    }
}
