mod model;
mod usecases;

use crate::usecases::{filter_to_movies, parse_to_csv_rows};
use csv::WriterBuilder;
use model::RyotExport;
use std::fs::File;

fn main() {
    let file = File::open("input.json").unwrap();

    let ryot_export: RyotExport = serde_json::from_reader(file).unwrap();
    let movies = filter_to_movies(ryot_export.metadata.as_ref().unwrap());

    let mut wtr = WriterBuilder::new()
        .has_headers(true)
        .from_path("movies.csv")
        .unwrap();

    for mut movie in movies {
        let records = parse_to_csv_rows(&mut movie);

        for record in records {
            wtr.serialize(&record)
                .expect(&format!("Failed serializing: {:?}", &record));
        }
    }
}
