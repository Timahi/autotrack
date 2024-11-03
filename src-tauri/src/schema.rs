// @generated automatically by Diesel CLI.

diesel::table! {
    profiles (id) {
        id -> Integer,
        name -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    vehicles (id) {
        id -> Integer,
        brand -> Text,
        model -> Text,
        odometer -> Integer,
        odometer_updated_at -> Timestamp,
        registration -> Text,
        registration_year -> Integer,
        serial_number -> Nullable<Text>,
        description -> Nullable<Text>,
        profile_id -> Integer,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    inspections (id) {
        id -> Integer,
        vehicle_id -> Integer,
        result -> Integer,
        performed_at -> Timestamp,
        next_at -> Timestamp,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    maintenance (id) {
        id -> Integer,
        vehicle_id -> Integer,
        #[sql_name = "type"]
        maintenance_type -> Text,
        description -> Nullable<Text>,
        odometer -> Integer,
        performed_at -> Timestamp,
        created_at -> Timestamp,
        updated_at -> Timestamp
    }
}

diesel::joinable!(vehicles -> profiles (profile_id));
diesel::joinable!(inspections -> vehicles (vehicle_id));
diesel::joinable!(maintenance -> vehicles (vehicle_id));

diesel::allow_tables_to_appear_in_same_query!(profiles, vehicles, inspections, maintenance,);
