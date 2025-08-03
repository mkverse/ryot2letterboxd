use crate::model::ExportMetadataMovieItem;

pub fn filter_to_movies(metadata: &[ExportMetadataMovieItem]) -> Vec<ExportMetadataMovieItem> {
    metadata
        .iter()
        .filter(|item| item.lot == "movie")
        .filter(|item| is_seen(item))
        .cloned()
        .collect()
}

fn is_seen(movie: &ExportMetadataMovieItem) -> bool {
    movie.seen_history.iter().filter(|h| h.is_some()).count() > 0
}
