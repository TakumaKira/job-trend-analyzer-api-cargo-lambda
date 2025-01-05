use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::env;
use dotenvy::dotenv;
use crate::aws_secrets_manager::get_secret::get_secret;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct DbSecrets {
    username: String,
    password: String,
    host: String,
    port: i32,
    dbname: String,
}

pub async fn establish_connection() -> PgPool {
    dotenv().ok();
    let aws_secret_name = env::var("AWS_DB_SECRETS_NAME").expect("AWS_SECRET_NAME must be set");

    let secrets: DbSecrets = serde_json::from_str(
        &get_secret(&aws_secret_name, None, None)
        .await
        .expect("Failed to get secret")
    )
        .expect("Failed to parse secrets JSON");
    println!("secrets: {:?}", secrets);

    let database_url = format!("postgres://{}:{}@{}:{}/{}", secrets.username, secrets.password, secrets.host, secrets.port, secrets.dbname);

    PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool")
}