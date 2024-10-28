// @generated automatically by Diesel CLI.

diesel::table! {
    profiles (id) {
        id -> Integer,
        name -> Text,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}
