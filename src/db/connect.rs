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
    port: String,
    dbname: String,
}

pub async fn establish_connection() -> PgPool {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("database_url: {}", database_url);
    let aws_secret_name = env::var("AWS_DB_SECRETS_NAME").expect("AWS_SECRET_NAME must be set");
    let secrets: DbSecrets = serde_json::from_str(&get_secret(&aws_secret_name, None, None).await.expect("Failed to get secret"))
        .expect("Failed to parse secrets JSON");
    println!("secrets: {:?}", secrets);

    let db_url = format!("postgres://{}:{}@{}:{}/{}", secrets.username, secrets.password, secrets.host, secrets.port, secrets.dbname);
    println!("db_url: {}", db_url);

    PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool")
}