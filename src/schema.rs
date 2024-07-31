// @generated automatically by Diesel CLI.
use diesel::{allow_tables_to_appear_in_same_query, joinable, table};
table! {
    candidate (id) {
        id -> Uuid,
        company_id -> Uuid,
        lastname -> Varchar,
        firstname -> Varchar,
        file_name -> Varchar,
        phone -> Varchar,
        email -> Varchar,
        motivation -> Text,
    }
}

table! {
    company (id) {
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
        company_id ->  Nullable<Uuid>,
        username -> Varchar,
        email -> Varchar,
        password -> Nullable<Varchar>,
        role -> Varchar,
    }
}

table! {
    job_offers (id) {
        id -> Uuid,
        company_id ->  Uuid,
        title -> Varchar,
        description -> Text,
        requirements -> Nullable<Varchar>,
        location -> Varchar,
        remote -> Nullable<Varchar>,
        employment_type -> Varchar,
        salary -> Float,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

joinable!(candidate -> company (company_id));
joinable!(users -> company (company_id));
joinable!(login_history -> users (user_id));
joinable!(job_offers -> company (company_id));

allow_tables_to_appear_in_same_query!(candidate, company, login_history, users, job_offers);
