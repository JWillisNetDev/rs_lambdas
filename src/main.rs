use lambda_runtime::{service_fn, Error, LambdaEvent};
use serde_json::{json, Value};


async fn handler(event: LambdaEvent<Value>) -> Result<Value, Error> {
    let payload = event.payload;
    let first_name = payload["first_name"].as_str().unwrap_or("world");

    Ok(json!(
        { "message": format!("Hello, {}!", first_name) }
    ))
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda_runtime::run(service_fn(handler)).await
}