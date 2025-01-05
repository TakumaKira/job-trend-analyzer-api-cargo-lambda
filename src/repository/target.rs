use sqlx::{Error, PgPool};
use crate::db::models::Target;

pub async fn get_targets(pool: &PgPool) -> Result<Vec<Target>, Error> {
    sqlx::query_as::<_, Target>(
        "SELECT * FROM targets"
    )
    .fetch_all(pool)
    .await
    .map_err(|err| {
        println!("Failed to get targets: {}", err);
        err
    })
}
