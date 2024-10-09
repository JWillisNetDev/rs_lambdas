use lambda_http::{service_fn, tracing, Body, Error, Request, RequestPayloadExt, Response};
// use base64::{Engine as _, engine::general_purpose};
// use serde_json::json;
use s3::{S3Client, PutFile};

mod s3;

async fn handler<T: PutFile>(
    event: Request,
    client: &T) -> Result<Response<Body>, Error> {
    let payload = match event.body() {
        Body::Binary(bytes) => bytes,
        _ => return Ok(Response::builder().status(400).body("Invalid body".into()).unwrap()),
    };

    client.put_file("s3-dithered-images", "test.png", "image/png", "base64", payload.clone()).await?;

    let resp = Response::builder()
        .status(200)
        .header("content-type", "text/html")
        .body("Hello AWS Lambda HTTP request".into())
        .map_err(Box::new)?;
    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    let shared_config = aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await;
    let client = S3Client::new(&shared_config);
    let client_ref = &client;

    let handler = service_fn(move |event| async move { handler(event, client_ref).await });

    lambda_http::run(handler).await?;

    Ok(())
}