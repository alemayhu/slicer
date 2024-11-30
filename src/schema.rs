// @generated automatically by Diesel CLI.

diesel::table! {
    runs (id) {
        id -> Int4,
        start_time -> Timestamp,
        end_time -> Nullable<Timestamp>,
        #[max_length = 50]
        status -> Varchar,
        description -> Nullable<Text>,
        parameters -> Nullable<Jsonb>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
