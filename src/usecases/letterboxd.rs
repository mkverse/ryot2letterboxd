use crate::model::{ExportMetadataMovieItem, LetterboxdImportItem, RyotSeenHistory};

pub fn parse_to_csv_rows(movie: &mut ExportMetadataMovieItem) -> Vec<LetterboxdImportItem> {
    movie.seen_history.sort_by_key(|opt_seen| match opt_seen {
        Some(item) => (false, item.ended_on.is_none(), item.ended_on.map(|d| d)),
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
        watched_date: seen_history.as_ref().map(|seen| seen.ended_on).flatten(),
        rating: None, // TODO: map rating
        rewatch,
    }
}
