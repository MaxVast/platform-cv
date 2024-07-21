// @generated automatically by Diesel CLI.

diesel::table! {
    login_history (id) {
        id -> Int4,
        user_id -> Int8,
        login_timestamp -> Timestamptz,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        email -> Varchar,
        password -> Varchar,
    }
}

diesel::joinable!(login_history -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    login_history,
    users,
);
