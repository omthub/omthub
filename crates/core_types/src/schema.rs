// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Text,
        name -> Text,
        email -> Text,
        pw_hash -> Text,
        is_active -> Bool,
        meta -> Text,
    }
}
