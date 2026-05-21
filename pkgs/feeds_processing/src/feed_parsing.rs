use crate::FeedsProcessingError;
use crate::{rss, atom};

pub struct Article {
    pub title: Option<String>,
}

pub fn parse_feed(content: &str) -> Result<Vec<Article>, FeedsProcessingError> {
    if let Ok(articles) = rss::parse(content) {
        return Ok(articles);
    }

    if let Ok(articles) = atom::parse(content) {
        return Ok(articles);
    }

    Err(FeedsProcessingError::Parsing(
        "Failed to parse feed as RSS or Atom".to_string(),
    ))
}
