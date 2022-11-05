// @generated automatically by Diesel CLI.

diesel::table! {
    note (id) {
        id -> Int4,
        ticket_id -> Nullable<Int4>,
        created_at -> Timestamp,
        author -> Nullable<Varchar>,
    }
}

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

diesel::joinable!(note -> ticket (ticket_id));

diesel::allow_tables_to_appear_in_same_query!(
    note,
    ticket,
);
