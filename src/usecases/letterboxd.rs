use crate::model::{ExportMetadataMovieItem, LetterboxdImportItem, RyotSeenHistory};

pub fn parse_to_csv_rows(movie: &mut ExportMetadataMovieItem) -> Vec<LetterboxdImportItem> {
    movie.seen_history.retain(|h| h.is_some());

    movie.seen_history.sort_by_key(|opt_seen| match opt_seen {
        Some(item) => (false, item.ended_on.is_none(), item.ended_on),
        None => (true, true, None),
    });

    movie
        .seen_history
        .iter()
        .enumerate()
        .map(|(index, seen_history_item)| parse_to_csv_row(movie, seen_history_item, index != 0))
        .collect()
}

fn parse_to_csv_row(
    movie: &ExportMetadataMovieItem,
    seen_history: &Option<RyotSeenHistory>,
    rewatch: bool,
) -> LetterboxdImportItem {
    LetterboxdImportItem {
        tmdb_id: movie.identifier.clone(),
        title: movie.source_id.clone(),
        watched_date: seen_history.as_ref().and_then(|seen| seen.ended_on),
        rating: None,
        rewatch,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::RyotSeenHistory;
    use chrono::{TimeZone, Utc};

    fn make_movie(seen_history: Vec<Option<RyotSeenHistory>>) -> ExportMetadataMovieItem {
        ExportMetadataMovieItem {
            lot: "movie".to_string(),
            source_id: "src123".to_string(),
            identifier: "12345".to_string(),
            source: "tmdb".to_string(),
            seen_history,
        }
    }

    fn seen_on(year: i32, month: u32, day: u32) -> Option<RyotSeenHistory> {
        Some(RyotSeenHistory {
            progress: "100".to_string(),
            ended_on: Some(Utc.with_ymd_and_hms(year, month, day, 0, 0, 0).unwrap()),
        })
    }

    fn seen_no_date() -> Option<RyotSeenHistory> {
        Some(RyotSeenHistory {
            progress: "100".to_string(),
            ended_on: None,
        })
    }

    #[test]
    fn none_entries_produce_no_rows() {
        let mut movie = make_movie(vec![None, seen_on(2024, 1, 1)]);
        let rows = parse_to_csv_rows(&mut movie);
        assert_eq!(rows.len(), 1);
        assert!(!rows[0].rewatch);
    }

    #[test]
    fn multiple_none_entries_produce_no_rows() {
        let mut movie = make_movie(vec![None, None, seen_on(2024, 6, 15)]);
        let rows = parse_to_csv_rows(&mut movie);
        assert_eq!(rows.len(), 1);
    }

    #[test]
    fn single_view_is_not_rewatch() {
        let mut movie = make_movie(vec![seen_on(2024, 1, 1)]);
        let rows = parse_to_csv_rows(&mut movie);
        assert_eq!(rows.len(), 1);
        assert!(!rows[0].rewatch);
    }

    #[test]
    fn second_view_is_rewatch() {
        let mut movie = make_movie(vec![seen_on(2024, 6, 1), seen_on(2024, 1, 1)]);
        let rows = parse_to_csv_rows(&mut movie);
        assert_eq!(rows.len(), 2);
        assert!(!rows[0].rewatch);
        assert!(rows[1].rewatch);
    }

    #[test]
    fn views_sorted_earliest_first() {
        let mut movie = make_movie(vec![
            seen_on(2024, 6, 1),
            seen_on(2023, 1, 1),
            seen_on(2024, 12, 31),
        ]);
        let rows = parse_to_csv_rows(&mut movie);
        assert_eq!(rows.len(), 3);
        let dates: Vec<_> = rows.iter().map(|r| r.watched_date.unwrap()).collect();
        assert!(dates[0] < dates[1]);
        assert!(dates[1] < dates[2]);
        assert!(!rows[0].rewatch);
        assert!(rows[1].rewatch);
        assert!(rows[2].rewatch);
    }

    #[test]
    fn views_without_date_come_after_dated_views() {
        let mut movie = make_movie(vec![seen_no_date(), seen_on(2024, 1, 1)]);
        let rows = parse_to_csv_rows(&mut movie);
        assert_eq!(rows.len(), 2);
        assert!(rows[0].watched_date.is_some());
        assert!(rows[1].watched_date.is_none());
    }
}
