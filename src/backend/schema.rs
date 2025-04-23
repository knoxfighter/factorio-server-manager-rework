// @generated automatically by Diesel CLI.

diesel::table! {
    users (uuid) {
        uuid -> Text,
        username -> Text,
        password -> Text,
    }
}
