use chrono::{DateTime,  Utc};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub struct RyotExport {
    pub metadata: Option<Vec<ExportMetadataMovieItem>>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub struct ExportMetadataMovieItem {
    pub lot: String,
    pub source_id: String,
    pub identifier: String,
    pub source: String,
    pub seen_history: Vec<Option<RyotSeenHistory>>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub struct RyotSeenHistory {
    pub progress: String,
    pub ended_on: Option<DateTime<Utc>>,
}