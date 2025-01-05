use job_trend_analyzer_api_cargo_lambda::{db::connect::establish_connection, repository::target::get_targets, response::build::build_response};
use lambda_http::{run, service_fn, Body, Error, Request, Response};

/// This is the main body for the function.
/// Write your code inside it.
/// There are some code example in the following URLs:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples
async fn function_handler(_event: Request) -> Result<Response<Body>, Error> {
    tracing_subscriber::fmt::init();

    let pool = establish_connection().await;
    let targets = get_targets(&pool).await?;

    let response_data = build_response(pool, targets).await;

    let json_string = serde_json::to_string_pretty(&response_data)
        .expect("Failed to serialize response data");

    Ok(Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(json_string.into())?)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(service_fn(function_handler)).await
}
