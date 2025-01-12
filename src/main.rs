use job_trend_analyzer_api_cargo_lambda::{db::connect::establish_connection, repository::target::get_targets, response::build::build_response};
use lambda_http::{run, service_fn, Body, Error, Request, Response, http::{Method, header::{ORIGIN, ACCESS_CONTROL_ALLOW_ORIGIN, ACCESS_CONTROL_ALLOW_METHODS, ACCESS_CONTROL_ALLOW_HEADERS, ACCESS_CONTROL_MAX_AGE, ACCESS_CONTROL_ALLOW_CREDENTIALS}}};
use std::env;
use tracing::{info, warn};

fn is_origin_allowed(origin: &str) -> bool {
    match env::var("ALLOWED_FRONTEND_ORIGINS") {
        Ok(allowed_origins) => {
            let origins: Vec<&str> = allowed_origins.split(',').map(|s| s.trim()).collect();
            let allowed = origins.contains(&origin);
            if allowed {
                info!(origin = origin, "Origin allowed");
            } else {
                warn!(origin = origin, allowed_origins = ?origins, "Origin not in allowed list");
            }
            allowed
        }
        Err(e) => {
            warn!(error = ?e, "Failed to get ALLOWED_FRONTEND_ORIGINS");
            false
        }
    }
}

/// This is the main body for the function.
/// Write your code inside it.
/// There are some code example in the following URLs:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples
async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    tracing_subscriber::fmt::init();

    info!(method = ?event.method(), "Handling request");
    
    // Check origin header
    let origin = event
        .headers()
        .get(ORIGIN)
        .and_then(|h| h.to_str().ok())
        .unwrap_or("");
    
    if !is_origin_allowed(origin) && !origin.is_empty() {
        warn!(origin = origin, "Forbidden request from disallowed origin");
        // Return 403 Forbidden for disallowed origins
        let error_response = serde_json::json!({
            "error": {
                "message": "Origin not allowed",
                "status": 403
            }
        });
        return Ok(Response::builder()
            .status(403)
            .header("content-type", "application/json")
            .body(Body::from(serde_json::to_string(&error_response)?))?);
    }

    // Handle preflight OPTIONS request
    if event.method() == Method::OPTIONS {
        info!("Handling preflight request");
        return Ok(Response::builder()
            .status(200)
            .header("content-type", "application/json")
            .header(ACCESS_CONTROL_ALLOW_METHODS, "GET, OPTIONS")
            .header(ACCESS_CONTROL_ALLOW_HEADERS, "content-type")
            .header(ACCESS_CONTROL_ALLOW_ORIGIN, if origin.is_empty() { "*" } else { origin })
            .header(ACCESS_CONTROL_MAX_AGE, "86400")
            .header(ACCESS_CONTROL_ALLOW_CREDENTIALS, "true")
            .body(Body::Empty)?);
    }

    // Normal response with CORS headers if origin is present
    let mut builder = Response::builder()
        .status(200)
        .header("content-type", "application/json");

    if !origin.is_empty() {
        builder = builder
            .header(ACCESS_CONTROL_ALLOW_ORIGIN, origin)
            .header(ACCESS_CONTROL_ALLOW_METHODS, "GET, OPTIONS")
            .header(ACCESS_CONTROL_ALLOW_HEADERS, "content-type")
            .header(ACCESS_CONTROL_ALLOW_CREDENTIALS, "true");
    }

    let pool = establish_connection().await;
    let targets = get_targets(&pool).await?;

    let response_data = build_response(pool, targets).await;
    let json_string = serde_json::to_string_pretty(&response_data)
        .expect("Failed to serialize response data");

    Ok(builder.body(json_string.into())?)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(service_fn(function_handler)).await
}
