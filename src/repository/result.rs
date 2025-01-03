use sqlx::PgPool;
use crate::db::models::Result;

pub async fn get_result(pool: &PgPool, target_url: String) -> anyhow::Result<Vec<Result>> {
    let results = sqlx::query_as::<_, Result>(
        "SELECT * FROM results WHERE url = $1"
    )
    .bind(target_url)
    .fetch_all(pool)
    .await?;

    Ok(results)
}
