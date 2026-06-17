use aws_config::{BehaviorVersion, Region};
use aws_lambda_events::s3::S3EventRecord;
use aws_sdk_s3::Client;
use aws_sdk_s3::primitives::ByteStream;
use bytes::Bytes;
use lambda_runtime::{Error, tracing};
use std::io::Cursor;

pub async fn create_configured_s3_client() -> Client {
    let region_str = std::env::var("AWS_REGION").unwrap_or_else(|_| "eu-west-1".to_string());
    let region = Region::new(region_str);
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

pub async fn get_file_from_record(
    record: &S3EventRecord,
    client: &Client,
) -> Result<Cursor<Bytes>, Error> {
    let bucket = record
        .s3
        .bucket
        .name
        .as_deref()
        .ok_or_else(|| Error::from("S3 event is missing bucket name"))?;
    let raw_key = record
        .s3
        .object
        .key
        .as_deref()
        .ok_or_else(|| Error::from("S3 event is missing object key"))?;
    let key = urlencoding::decode(raw_key)
        .map_err(|e| Error::from(format!("Failed to decode S3 key '{raw_key}': {e}")))?;

    tracing::info!("Loading file '{}' from bucket '{}'", key, bucket);
    let resp = client
        .get_object()
        .bucket(bucket)
        .key(key.as_ref())
        .send()
        .await?;
    let body = resp.body.collect().await?.into_bytes();
    Ok(Cursor::new(body))
}

pub async fn put_file_to_s3(
    client: &Client,
    buffer: Vec<u8>,
    bucket: &str,
    key: &str,
) -> Result<(), Error> {
    tracing::info!("Saving file '{}' to bucket '{}'", key, bucket);
    client
        .put_object()
        .bucket(bucket)
        .key(key)
        .body(ByteStream::from(buffer))
        .send()
        .await
        .map_err(|e| {
            Error::from(format!(
                "Failed to upload '{key}' to S3 bucket '{bucket}': {e}"
            ))
        })?;
    Ok(())
}
