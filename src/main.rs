mod aws;
mod model;
mod usecases;

use crate::aws::{create_configured_s3_client, get_file_from_record, put_file_to_s3};
use crate::usecases::{filter_to_movies, parse_to_csv_rows};
use aws_lambda_events::s3::{S3Event, S3EventRecord};
use aws_sdk_s3::Client;
use chrono::Utc;
use csv::WriterBuilder;
use lambda_runtime::{Error, LambdaEvent, run, service_fn, tracing};
use model::RyotExport;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();
    run(service_fn(|request: LambdaEvent<S3Event>| handler(request))).await
}

pub(crate) async fn handler(event: LambdaEvent<S3Event>) -> Result<(), Error> {
    if event.payload.records.is_empty() {
        return Err(Error::from("No S3 event records found"));
    }

    let out_bucket = env::var("OUT_BUCKET").map_err(|_| Error::from("OUT_BUCKET not set"))?;
    let out_prefix = env::var("OUT_PREFIX").map_err(|_| Error::from("OUT_PREFIX not set"))?;

    let s3_client = create_configured_s3_client().await;

    for record in &event.payload.records {
        process_record(record, &s3_client, &out_bucket, &out_prefix).await?;
    }

    Ok(())
}

async fn process_record(
    record: &S3EventRecord,
    s3_client: &Client,
    out_bucket: &str,
    out_prefix: &str,
) -> Result<(), Error> {
    let mut file = get_file_from_record(record, s3_client).await?;

    let ryot_export: RyotExport = serde_json::from_reader(&mut file)
        .map_err(|e| Error::from(format!("Failed to parse Ryot export JSON: {e}")))?;
    let metadata = ryot_export.metadata.as_deref().unwrap_or_default();
    let movies = filter_to_movies(metadata);

    let mut buffer = Vec::new();
    {
        let mut wtr = WriterBuilder::new()
            .has_headers(true)
            .from_writer(&mut buffer);

        for mut movie in movies {
            let rows = parse_to_csv_rows(&mut movie);
            for item in rows {
                wtr.serialize(&item)
                    .map_err(|e| Error::from(format!("Failed serializing record {item:?}: {e}")))?;
            }
        }
    }

    let timestamp = Utc::now().format("%Y%m%d-%H%M%S");
    put_file_to_s3(
        s3_client,
        buffer,
        out_bucket,
        &format!("{out_prefix}/movies_{timestamp}.csv"),
    )
    .await
}
