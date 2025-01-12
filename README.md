# About this app

This project uses [Cargo Lambda](https://www.cargo-lambda.info), aiming to serve API functionality on AWS Lambda.
Before examing this project, you need to setup your terminal to be able to run `cargo` and `cargo lambda` commands.

## Setup

1. Install [Rust](https://www.rust-lang.org/tools/install)
2. Install [Cargo Lambda](https://www.cargo-lambda.info/guide/getting-started.html)
3. (For local development) Prepare .env file with DATABASE_URL, which is the connection string to your database.
(For AWS Lambda) Prepare AWS_DB_SECRETS_NAME, which is the name of the secret in AWS Secrets Manager. The secret should contain username, password, host, port, dbname.

## Run this app

```bash
LOCAL=true cargo lambda watch
```

From another terminal, run the following command to test the app:

```bash
cargo lambda invoke --data-file fixtures/apigw-request-base.json
```

The request data file is basically a copy from [the AWS official fixture](https://github.com/awslabs/aws-lambda-rust-runtime/blob/main/lambda-events/src/fixtures/example-apigw-request.json).
You can test another API Gateway request by editing the file or copying this file and modify it.

## Deploy this app

### Things you need to deploy this app to AWS

- Your function needs to have the following environment variables.
    - AWS_DB_SECRETS_NAME: Secret name on Secret Manager which contains `username` and `password` of your database.
    - DB_HOST: Your database's endpoint
    - DB_NAME: Your database's name
    - DB_PORT: Your database's port(highly likely `5432` as this app uses PostgreSQL)
    - ALLOWED_FRONTEND_ORIGINS: Provide frontend origins to allow access with comma separated strings. If the frontend that has an origin it does not include send request to this API, browsers will reject the response with CORS error.
- Your function have access to the following:
    - Your database(e.g. in the same VPC if your database don't have public endpoint for the sake of security)
    - AWS's Secret Manager service(e.g. VPC endpoint or internet access)
