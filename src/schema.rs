// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        last_name -> Varchar,
        email -> Varchar,
        password -> Varchar,
        age -> Nullable<Int4>,
        address -> Nullable<Varchar>,
        created_at -> Timestamp,
        modified_at -> Timestamp,
    }
}
