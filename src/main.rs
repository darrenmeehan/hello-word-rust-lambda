#[macro_use]
extern crate log;
extern crate simple_logger;
use lambda::lambda;
use serde_json::Value;

type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

#[lambda]
#[tokio::main]
async fn main(event: Value) -> Result<Value, Error> {
    simple_logger::init().unwrap();
    debug!("This is an example debug message.");
    info!("This is an example info message.");
    warn!("This is an example warn message.");
    error!("This is an example error message.");
    Ok(event)
}
