use job_trend_analyzer_api_cargo_lambda::{db, repository};
use lambda_http::{run, service_fn, tracing, Body, Error, Request, Response};

/// This is the main body for the function.
/// Write your code inside it.
/// There are some code example in the following URLs:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples
async fn function_handler(_event: Request) -> Result<Response<Body>, Error> {
    let mut conn = db::connect::establish_connection();
    let targets = repository::target::get_targets(&mut conn);

    // Create the combined data structure
    let combined_data: Vec<serde_json::Value> = targets
        .iter()
        .map(|target| {
            let results = repository::result::get_result(&mut conn, target.url.clone());
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
        })
        .collect();

    // Convert to JSON string if needed
    let json_string = serde_json::to_string_pretty(&combined_data).unwrap();

    // Return something that implements IntoResponse.
    // It will be serialized to the right response event automatically by the runtime
    let resp = Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(json_string.into())
        .map_err(Box::new)?;
    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    run(service_fn(function_handler)).await
}
