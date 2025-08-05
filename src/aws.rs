use aws_config::{BehaviorVersion, Region};
use aws_lambda_events::s3::S3Event;
use aws_sdk_s3::primitives::ByteStream;
use aws_sdk_s3::Client;
use lambda_runtime::{tracing, Error, LambdaEvent};
use std::io::Cursor;

pub async fn create_configured_s3_client() -> Client {
    let region = Region::new("eu-west-1".to_string());
    let config = aws_config::defaults(BehaviorVersion::v2025_01_17())
        .region(region)
        .timeout_config(
            aws_smithy_types::timeout::TimeoutConfig::builder()
                .connect_timeout(std::time::Duration::from_secs(5))
                .build(),
        )
        .load()
        .await;
    Client::new(&config)
}

pub async fn get_file_from_event(
    event: LambdaEvent<S3Event>,
    client: &Client,
) -> Result<Cursor<Vec<u8>>, Error> {
    let bucket = event.payload.records[0]
        .s3
        .bucket
        .name
        .as_ref()
        .expect("Bucket name to exist");
    let key = event.payload.records[0]
        .s3
        .object
        .key
        .as_ref()
        .expect("Object key name to exist");

    tracing::info!("Loading file '{}' from bucket '{}'", key, bucket);
    let resp = client.get_object().bucket(bucket).key(key).send().await?;
    let body = resp.body.collect().await?.into_bytes();
    Ok(Cursor::new(body.to_vec()))
}

pub async fn put_file_to_s3(client: Client, buffer: Vec<u8>, bucket: &str, key: &str) {
    tracing::info!("Saving file '{}' to bucket '{}'", key, bucket);
    client
        .put_object()
        .bucket(bucket)
        .key(key)
        .body(ByteStream::from(buffer))
        .send()
        .await
        .expect("Failed to upload to S3");
}
