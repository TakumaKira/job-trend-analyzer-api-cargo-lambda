use serde::Serialize;

#[derive(Serialize)]
pub struct ResponseItem {
    pub url: String,
    pub results: Vec<ResultItem>,
}

#[derive(Serialize)]
pub struct ResultItem {
    pub job_title: String,
    pub job_location: String,
    pub scrape_date: String,
    pub count: i32,
}
