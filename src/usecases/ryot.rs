use crate::model::ExportMetadataMovieItem;

pub fn filter_to_movies(metadata: &[ExportMetadataMovieItem]) -> Vec<ExportMetadataMovieItem> {
    metadata
        .iter()
        .filter(|item| item.lot.eq_ignore_ascii_case("movie"))
        .filter(|item| is_seen(item))
        .cloned()
        .collect()
}

fn is_seen(movie: &ExportMetadataMovieItem) -> bool {
    movie.seen_history.iter().any(|h| h.is_some())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::RyotSeenHistory;

    fn make_movie(
        lot: &str,
        seen_history: Vec<Option<RyotSeenHistory>>,
    ) -> ExportMetadataMovieItem {
        ExportMetadataMovieItem {
            lot: lot.to_string(),
            source_id: "src123".to_string(),
            identifier: "12345".to_string(),
            source: "tmdb".to_string(),
            seen_history,
        }
    }

    fn seen() -> Option<RyotSeenHistory> {
        Some(RyotSeenHistory {
            progress: "100".to_string(),
            ended_on: None,
        })
    }

    #[test]
    fn includes_movie_regardless_of_case() {
        let items = vec![
            make_movie("movie", vec![seen()]),
            make_movie("Movie", vec![seen()]),
            make_movie("MOVIE", vec![seen()]),
        ];
        let result = filter_to_movies(&items);
        assert_eq!(result.len(), 3);
    }

    #[test]
    fn excludes_non_movie_lot() {
        let items = vec![
            make_movie("show", vec![seen()]),
            make_movie("book", vec![seen()]),
            make_movie("movie", vec![seen()]),
        ];
        let result = filter_to_movies(&items);
        assert_eq!(result.len(), 1);
    }

    #[test]
    fn excludes_movie_with_no_some_seen_history() {
        let items = vec![
            make_movie("movie", vec![None, None]),
            make_movie("movie", vec![seen()]),
        ];
        let result = filter_to_movies(&items);
        assert_eq!(result.len(), 1);
    }
}
