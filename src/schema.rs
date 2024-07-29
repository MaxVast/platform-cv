// @generated automatically by Diesel CLI.
use diesel::{allow_tables_to_appear_in_same_query, joinable, table};
table! {
    candidate (id) {
        id -> Uuid,
        entreprise_id -> Uuid,
        lastname -> Varchar,
        firstname -> Varchar,
        file_name -> Varchar,
        motivation -> Text,
    }
}

table! {
    entreprise (id) {
        id -> Uuid,
        name -> Varchar,
    }
}

table! {
    login_history (id) {
        id -> Uuid,
        user_id -> Uuid,
        login_timestamp -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Uuid,
        username -> Varchar,
        email -> Varchar,
        password -> Nullable<Varchar>,
        role -> Varchar
    }
}

joinable!(candidate -> entreprise (entreprise_id));
joinable!(login_history -> users (user_id));

allow_tables_to_appear_in_same_query!(
    candidate,
    entreprise,
    login_history,
    users,
);
