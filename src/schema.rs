// @generated automatically by Diesel CLI.

diesel::table! {
    ticket (id) {
        id -> Int4,
        email -> Varchar,
        description -> Varchar,
        created_at -> Timestamp,
        assigned_to -> Nullable<Varchar>,
        status -> Varchar,
        reporter -> Nullable<Varchar>,
        reporter_email -> Nullable<Varchar>,
        severity -> Varchar,
        reporter_estimate -> Int4,
    }
}
