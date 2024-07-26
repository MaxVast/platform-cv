// @generated automatically by Diesel CLI.

diesel::table! {
    candidate (id) {
        id -> Uuid,
        entreprise_id -> Uuid,
        lastname -> Varchar,
        firstname -> Varchar,
        file_name -> Varchar,
        motivation -> Text,
    }
}

diesel::table! {
    entreprise (id) {
        id -> Uuid,
        name -> Varchar,
    }
}

diesel::table! {
    login_history (id) {
        id -> Uuid,
        user_id -> Uuid,
        login_timestamp -> Timestamptz,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        username -> Varchar,
        email -> Varchar,
        password -> Varchar,
    }
}

diesel::joinable!(candidate -> entreprise (entreprise_id));
diesel::joinable!(login_history -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    candidate,
    entreprise,
    login_history,
    users,
);
