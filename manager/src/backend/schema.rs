// @generated automatically by Diesel CLI.

diesel::table! {
    instances (uuid) {
        uuid -> Integer,
        name -> Text,
        factorio_version -> Text,
    }
}

diesel::table! {
    users (uuid) {
        uuid -> Text,
        username -> Text,
        password -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(instances, users,);
