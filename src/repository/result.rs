use sqlx::{PgPool, Error};
use crate::db;

pub async fn get_result(pool: &PgPool, target_url: String) -> Result<Vec<db::models::Result>, Error> {
    sqlx::query_as::<_, db::models::Result>(
        "SELECT * FROM results WHERE url = $1"
    )
    .bind(target_url)
    .fetch_all(pool)
    .await
    .map_err(|err| {
        println!("Failed to get results: {}", err);
        err
    })
}