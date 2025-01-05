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
