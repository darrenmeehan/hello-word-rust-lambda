#!/usr/bin/env bash

-e

cargo check

cargo test

cargo clippy

cargo build --release --target x86_64-unknown-linux-musl

cp ./target/x86_64-unknown-linux-musl/release/register-on-upload ./bootstrap && zip lambda.zip bootstrap && rm bootstrap

aws lambda update-function-code --function-name registerOnUpload \
--zip-file fileb://./lambda.zip

aws lambda invoke --function-name registerOnUpload \
  --cli-binary-format raw-in-base64-out \
  --payload '{"firstName": "world"}' \
  output.json

cat output.json
