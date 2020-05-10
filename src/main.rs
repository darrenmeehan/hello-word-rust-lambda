// #![feature(type_ascription)]
#[macro_use]
extern crate log;
extern crate simple_logger;
use lambda::lambda;
use serde_json::Value;
use rusoto_core::Region;
use rusoto_dynamodb::{AttributeValue, DynamoDbClient, DynamoDb, PutItemInput};
use std::collections::HashMap;

type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

#[lambda]
#[tokio::main]
async fn main(event: Value) -> Result<Value, Error> {
    simple_logger::init().unwrap();

    debug!("event={}", event);
    debug!("This is an example debug message.");
    info!("This is an example info message.");
    warn!("This is an example warn message.");
    error!("This is an example error message.");

    let table_name: String = "photos".to_string();
    let av = AttributeValue {
        s: Option::from(event.to_string()),
        ..Default::default()
    };
    let mut item: HashMap<String, AttributeValue> = HashMap::new();
    item.insert("testing".to_string(), av);
    let input = PutItemInput {
        item,
        table_name,
        ..Default::default()
    };

    let client = DynamoDbClient::new(Region::EuWest1);
    let response = client.put_item(input);

    // debug!("response={:?}", response);
    Ok(event)
}
