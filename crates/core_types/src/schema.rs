// @generated automatically by Diesel CLI.

diesel::table! {
    sessions (id) {
        id -> Text,
        data -> Bytea,
        expiry_date -> Timestamptz,
    }
}

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

diesel::allow_tables_to_appear_in_same_query!(sessions, users,);
