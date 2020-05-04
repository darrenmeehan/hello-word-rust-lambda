# register-on-upload

Simple AWS Lambda to document sent images in a DB.

## Test

```commandline
cargo test
```

## Build

To build the Rust binary

```commandline
$ cargo build --release
```

### Build on MacOS

```commandline
$ cargo build --release --target x86_64-unknown-linux-mus
```

To build the AWS Lambda zip file

```commandline
$ cp ./target/x86_64-unknown-linux-musl/release/register-on-upload ./bootstrap && zip lambda.zip bootstrap && rm bootstrap
```

## Deployment

Create AWS Lambda for the first time

```commandline
$ aws lambda create-function --function-name registerOnUpload \
  --handler not.used.in.rust \
  --zip-file fileb://./lambda.zip \
  --runtime provided \
  --role arn:aws:iam::XXXXXXXXXXXXX:role/your_lambda_execution_role \
  --environment Variables={RUST_BACKTRACE=1} \
  --tracing-config Mode=Active
```

To update the code of an existing AWS Lambda

```commandline
$ aws lambda update-function-code --function-name registerOnUpload \
--zip-file fileb://./lambda.zip
```

## Invoke AWS Lambda

To test your deployed AWS Lambda works correctly

```commandline
$ aws lambda invoke --function-name registerOnUpload \
  --cli-binary-format raw-in-base64-out \
  --payload '{"firstName": "world"}' \
  output.json
$ cat output.json  # Prints: {"message":"Hello, world!"}
```
