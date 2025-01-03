use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow)]
pub struct Result {
    pub id: i32,
    pub url: String,
    pub job_title: String,
    pub job_location: String,
    pub scrape_date: NaiveDateTime,
    pub count: i32,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct Target {
    pub id: i32,
    pub url: String,
}