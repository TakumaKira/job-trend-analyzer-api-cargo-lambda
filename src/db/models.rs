use diesel::prelude::*;
use chrono::NaiveDateTime;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::db::schema::results)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Result {
    pub id: i32,
    pub url: String,
    pub job_title: String,
    pub job_location: String,
    pub scrape_date: NaiveDateTime,
    pub count: i32,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::db::schema::targets)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Target {
    pub id: i32,
    pub url: String,
}