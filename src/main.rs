use job_trend_analyzer_api_cargo_lambda::{db, repository};
use lambda_http::{run, service_fn, Body, Error, Request, Response};
use futures::future;

/// This is the main body for the function.
/// Write your code inside it.
/// There are some code example in the following URLs:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples
async fn function_handler(_event: Request) -> Result<Response<Body>, Error> {
    let pool = db::connect::establish_connection().await;
    let targets = repository::target::get_targets(&pool).await?;

    let combined_data: Vec<serde_json::Value> = future::join_all(
        targets.iter().map(|target| {
            let target_url = target.url.clone();
            let pool = pool.clone();
            async move {
                let results = repository::result::get_result(&pool, target_url)
                    .await
                    .unwrap_or_default();
                
                let matching_results: Vec<serde_json::Value> = results
                    .iter()
                    .filter(|result| result.url == target.url)
                    .map(|result| serde_json::json!({
                        "job_title": result.job_title,
                        "job_location": result.job_location,
                        "scrape_date": result.scrape_date.to_string(),
                        "count": result.count
                    }))
                    .collect();

                serde_json::json!({
                    "url": target.url,
                    "results": matching_results
                })
            }
        })
    ).await;

    let json_string = serde_json::to_string_pretty(&combined_data)?;

    Ok(Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(json_string.into())?)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(service_fn(function_handler)).await
}
