#[macro_use]
extern crate log;
extern crate simple_logger;
use lambda::lambda;
use rusoto_core::Region;
use rusoto_dynamodb::{AttributeValue, DynamoDb, DynamoDbClient, PutItemInput};
use serde_json::Value;
use std::collections::HashMap;

type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

#[lambda]
#[tokio::main]
async fn main(event: Value) -> Result<Value, Error> {
    handler(event).await
}

async fn handler(event: Value) -> Result<Value, Box<Error>> {
    simple_logger::init().unwrap();

    // Check event is s3 notification
    // Should this be so specific?
    debug!("event={}", event);
    // FIXME Add support for more than one record..
    let s3_object = event["Records"][0]["s3"]["object"].to_string();
    let id = event["Records"][0]["s3"]["object"]["key"].to_string();
    debug!("id={}", id);

    example_logging();

    // FIXME Get table name from environment variable
    let table_name: String = "photos".to_string();
    let details = AttributeValue {
        s: Option::from(s3_object),
        ..Default::default()
    };
    let id_value = AttributeValue {
        s: Option::from(id),
        ..Default::default()
    };

    let mut item: HashMap<String, AttributeValue> = HashMap::new();
    item.insert("id".to_string(), id_value);
    item.insert("details".to_string(), details);
    let input = PutItemInput {
        item,
        table_name,
        ..Default::default()
    };
    let client = DynamoDbClient::new(Region::EuWest1);
    let response = client.put_item(input).await;

    let response = match response {
        Ok(entry) => debug!("Added entry"), // FIXME Figure out how to log entry
        // FIXME Fail Lambda run on error
        Err(error) => panic!("Problem adding to DB: {}", error),
    };
    Ok(event)
}

fn example_logging() {
    debug!("This is an example debug message.");
    info!("This is an example info message.");
    warn!("This is an example warn message.");
    error!("This is an example error message.");
}
