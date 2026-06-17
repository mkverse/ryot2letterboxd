# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What This Is

An AWS Lambda function (Rust) that converts [Ryot](https://ryot.io) JSON movie watch history exports into [Letterboxd](https://letterboxd.com)-compatible CSV files. It is triggered by S3 object creation events.

## Commands

```bash
# Build for Lambda (ARM64) — required for deployment
cargo lambda build --release --arm64

# Deploy to AWS
cargo lambda deploy --region eu-west-1 --env-vars OUT_BUCKET=<bucket>,OUT_PREFIX=letterboxd

# Run tests
cargo test

# Lint (CI treats warnings as errors)
cargo clippy -- -D warnings

# Format check
cargo fmt --check

# Format in place
cargo fmt
```

## Architecture

```
S3 trigger (JSON upload)
  └─ Lambda handler (src/main.rs)
        ├─ aws.rs          — S3 read/write helpers + client config
        ├─ model/
        │    ├─ ryot_export.rs       — Ryot JSON deserialization structs
        │    └─ letterboxd_import.rs — Letterboxd CSV serialization structs
        └─ usecases/
             ├─ ryot.rs       — filter_to_movies: keeps only watched movies
             └─ letterboxd.rs — parse_to_csv_rows: converts viewings to CSV rows
```

**Data flow:** S3 event → (loop over records) → `get_file_from_record` → `serde_json` parse → `filter_to_movies` → `parse_to_csv_rows` → CSV buffer → `put_file_to_s3`.

**Key mapping:** `RyotExport.metadata[]` items with `lot == "movie"` and non-empty `seen_history` → `LetterboxdImportItem` rows. Each view in `seen_history` becomes its own CSV row; the first (chronologically earliest) view has `rewatch = false`, all subsequent ones have `rewatch = true`.

## Environment Variables

| Variable | Required | Default | Purpose |
|---|---|---|---|
| `OUT_BUCKET` | yes | — | S3 bucket for output CSV |
| `OUT_PREFIX` | yes | — | Key prefix for output files |
| `AWS_REGION` | no | `eu-west-1` | S3 client region |

## Known Gaps

- Ratings are not mapped (`Rating10` is always empty) — Ryot and Letterboxd use different scales.
- Only movies are exported; TV shows and other media are ignored.
- The Ryot export does not include a human-readable title; the CSV `Title` column contains `source_id` (an external provider ID). Letterboxd can still match the film via `tmdbID`.
