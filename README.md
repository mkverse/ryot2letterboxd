# ryot2letterboxd

An AWS Lambda function that converts [Ryot](https://ryot.io) movie watch history exports into [Letterboxd](https://letterboxd.com)-compatible CSV files.

## Overview

When you upload a Ryot JSON export to a source S3 bucket, this Lambda function:

1. Reads and parses the Ryot export JSON
2. Filters to watched movies only
3. Converts each viewing (including rewatches) into a Letterboxd CSV row
4. Uploads the resulting CSV to a destination S3 bucket

You can then import the CSV directly into Letterboxd via **Settings > Import & Export > Import Films**.

## Architecture

```
S3 (Ryot JSON export)
    └─ triggers Lambda
            ├─ filters movies with watch history
            ├─ converts viewings to Letterboxd CSV rows
            └─ S3 (output CSV)
```

## Prerequisites

- AWS account with permissions to create Lambda functions and S3 buckets
- Rust toolchain (for building from source)
- [`cargo-lambda`](https://www.cargo-lambda.info/) for building and deploying

## Building

```bash
# Install cargo-lambda
cargo install cargo-lambda

# Build for Lambda (ARM64)
cargo lambda build --release --arm64
```

## Deployment

### 1. Create S3 buckets

Create an input bucket (for Ryot exports) and an output bucket (for generated CSVs), or reuse existing ones.

### 2. Deploy the Lambda function

```bash
cargo lambda deploy \
  --region eu-west-1 \
  --env-vars OUT_BUCKET=<your-output-bucket>,OUT_PREFIX=letterboxd
```

### 3. Configure S3 trigger

In the AWS Console (or via CLI/IaC), add an S3 trigger on the input bucket to invoke this Lambda when a `.json` object is created.

### 4. Set environment variables

| Variable | Description |
|----------|-------------|
| `OUT_BUCKET` | S3 bucket name where the output CSV will be written |
| `OUT_PREFIX` | Key prefix (folder path) for output files, e.g. `letterboxd` |
| `AWS_REGION` | AWS region for the S3 client (default: `eu-west-1`) |

## Usage

1. Export your data from Ryot: **Settings > Export > JSON**
2. Upload the exported JSON to the input S3 bucket
3. The Lambda triggers automatically and writes `movies_YYYYMMDD-HHMMSS.csv` to the output bucket
4. Download the CSV and import it into Letterboxd: **Settings > Import & Export > Import Films**

## CSV Format

The output CSV follows Letterboxd's import schema:

| Column | Source |
|--------|--------|
| `tmdbID` | Ryot `identifier` field |
| `Title` | Ryot `source_id` field |
| `WatchedDate` | Ryot `ended_on` field (YYYY-MM-DD) |
| `Rating10` | Not yet mapped (always empty) |
| `Rewatch` | `true` for viewings after the first |

## Development

```bash
# Run tests
cargo test

# Check for issues
cargo clippy

# Format code
cargo fmt
```

## Known Limitations

- **Ratings are not mapped** -- Ryot and Letterboxd use different rating scales; this mapping is not yet implemented.
- **Only movies are exported** -- TV shows and other media types are ignored.

## Contributing

Contributions are welcome. Please open an issue to discuss significant changes before submitting a pull request.

1. Fork the repository
2. Create a feature branch (`git checkout -b feat/my-feature`)
3. Commit your changes
4. Open a pull request

## License

MIT -- see [LICENSE](LICENSE).
