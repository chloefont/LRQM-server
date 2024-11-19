// @generated automatically by Diesel CLI.

diesel::table! {
    events (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        start_date -> Timestamp,
        end_date -> Timestamp,
        meters_goal -> Int4,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 255]
        username -> Varchar,
        #[max_length = 100]
        bib_id -> Varchar,
        event_id -> Int4,
        total_meters -> Int4,
    }
}

diesel::joinable!(users -> events (event_id));

diesel::allow_tables_to_appear_in_same_query!(
    events,
    users,
);
