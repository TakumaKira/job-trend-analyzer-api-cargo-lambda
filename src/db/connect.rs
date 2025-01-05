use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::env;
use dotenvy::dotenv;
use crate::aws_secrets_manager::get_secret::get_secret;

pub async fn establish_connection() -> PgPool {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let aws_secret_name = env::var("AWS_DB_SECRETS_NAME").expect("AWS_SECRET_NAME must be set");
    let secret = get_secret(&aws_secret_name, None, None).await.expect("Failed to get secret");
    println!("secret: {}", secret);

    PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool")
}