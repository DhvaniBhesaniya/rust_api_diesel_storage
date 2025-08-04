// @generated automatically by Diesel CLI.

diesel::table! {
    tasks (id) {
        id -> Integer,
        title -> Text,
        completed -> Bool,
        created_at -> Timestamp,
    }
}