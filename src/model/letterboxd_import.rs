use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct LetterboxdImportItem {
    #[serde(rename = "tmdbID")]
    pub tmdb_id: String,
    pub title: String,
    #[serde(with = "option_date_format")]
    pub watched_date: Option<DateTime<Utc>>,
    pub rating: Option<String>,
    pub rewatch: bool,
}

mod option_date_format {
    use chrono::{DateTime, NaiveDate, Utc};
    use serde::{self, Deserialize, Deserializer, Serializer};

    const FORMAT: &str = "%Y-%m-%d";

    pub fn serialize<S>(date: &Option<DateTime<Utc>>, ser: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match date {
            Some(dt) => ser.serialize_str(&dt.format(FORMAT).to_string()),
            None => ser.serialize_none(),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<DateTime<Utc>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let opt = Option::<String>::deserialize(deserializer)?;
        opt.map(|s| {
            NaiveDate::parse_from_str(&s, FORMAT)
                .map_err(serde::de::Error::custom)
                .and_then(|date| {
                    Ok(DateTime::<Utc>::from_naive_utc_and_offset(
                        date.and_hms_opt(0, 0, 0).expect("REASON"),
                        Utc,
                    ))
                })
        })
        .transpose()
    }
}
