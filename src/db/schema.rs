// @generated automatically by Diesel CLI.

diesel::table! {
    results (id) {
        id -> Int4,
        url -> Varchar,
        job_title -> Varchar,
        job_location -> Varchar,
        scrape_date -> Timestamp,
        count -> Int4,
    }
}

diesel::table! {
    targets (id) {
        id -> Int4,
        url -> Varchar,
        job_title -> Varchar,
        job_location -> Varchar,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    results,
    targets,
);
