mod aws;
mod model;
mod usecases;

use crate::aws::{create_configured_s3_client, get_file_from_event, put_file_to_s3};
use crate::usecases::{filter_to_movies, parse_to_csv_rows};
use aws_lambda_events::s3::S3Event;
use chrono::Utc;
use csv::WriterBuilder;
use lambda_runtime::{run, service_fn, tracing, Error, LambdaEvent};
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

    let s3_client = create_configured_s3_client().await;

    let mut file = get_file_from_event(event, &s3_client).await?;

    let ryot_export: RyotExport = serde_json::from_reader(&mut file).unwrap();
    let movies = filter_to_movies(ryot_export.metadata.as_ref().unwrap());

    let mut buffer = Vec::new();
    {
        let mut wtr = WriterBuilder::new()
            .has_headers(true)
            .from_writer(&mut buffer);

        for mut movie in movies {
            let records = parse_to_csv_rows(&mut movie);

            for record in records {
                wtr.serialize(&record)
                    .expect(&format!("Failed serializing: {:?}", &record));
            }
        }
    }

    let out_bucket = env::var("OUT_BUCKET").map_err(|_| Error::from("OUT_BUCKET not set"))?;
    let out_prefix = env::var("OUT_PREFIX").map_err(|_| Error::from("OUT_PREFIX not set"))?;
    let timestamp = Utc::now().format("%Y%m%d-%H%M%S");

    put_file_to_s3(
        s3_client,
        buffer,
        out_bucket.as_str(),
        format!("{out_prefix}/movies_{timestamp}.csv").as_str(),
    )
    .await;

    Ok(())
}
