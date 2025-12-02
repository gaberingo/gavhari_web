// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Int4,
        name -> Nullable<Varchar>,
        username -> Varchar,
        email -> Nullable<Varchar>,
        password -> Varchar,
    }
}
