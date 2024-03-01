// @generated automatically by Diesel CLI.

diesel::table! {
    config (uuid) {
        uuid -> Text,
        config_data -> Text,
    }
}

diesel::table! {
    requesttabs (uuid) {
        uuid -> Text,
        tabdata -> Text,
        tabdata_saved -> Nullable<Text>,
        saved_timestamp -> Nullable<Timestamp>,
    }
}

diesel::table! {
    requesttabs_sessions (uuid) {
        uuid -> Text,
        requesttabs_uuid -> Nullable<Text>,
        sessions_uuid -> Nullable<Text>,
    }
}

diesel::table! {
    sessions (uuid) {
        uuid -> Text,
    }
}

diesel::joinable!(requesttabs_sessions -> requesttabs (requesttabs_uuid));
diesel::joinable!(requesttabs_sessions -> sessions (sessions_uuid));

diesel::allow_tables_to_appear_in_same_query!(
    config,
    requesttabs,
    requesttabs_sessions,
    sessions,
);
