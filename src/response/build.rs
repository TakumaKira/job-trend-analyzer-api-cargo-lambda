use futures::future;
use sqlx::PgPool;
use crate::{db::models::Target, repository::result::get_result, response::models::{ResponseItem, ResultItem}};

pub async fn build_response(pool: PgPool, targets: Vec<Target>) -> Vec<ResponseItem> {
    future::join_all(
        targets.iter().map(|target| {
            let target_url = target.url.clone();
            let pool = pool.clone();
            async move {
                let results = get_result(&pool, target_url)
                    .await
                    .unwrap_or_default();
                
                let matching_results: Vec<ResultItem> = results
                    .iter()
                    .filter(|result| result.url == target.url)
                    .map(|result| ResultItem {
                        job_title: result.job_title.clone(),
                        job_location: result.job_location.clone(),
                        scrape_date: result.scrape_date.to_string(),
                        count: result.count,
                    })
                    .collect();

                ResponseItem {
                    url: target.url.clone(),
                    results: matching_results
                }
            }
        })
    ).await
}