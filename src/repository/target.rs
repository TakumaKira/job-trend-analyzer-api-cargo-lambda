use sqlx::PgPool;
use crate::db::models::Target;

pub async fn get_targets(pool: &PgPool) -> anyhow::Result<Vec<Target>> {
    let targets = sqlx::query_as::<_, Target>(
        "SELECT * FROM targets"
    )
    .fetch_all(pool)
    .await?;

    Ok(targets)
}
