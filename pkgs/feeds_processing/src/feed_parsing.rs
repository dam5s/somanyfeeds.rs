use chrono::{DateTime, Utc};
use crate::FeedsProcessingError;
use crate::{rss, atom};

pub struct Article {
    pub title: Option<String>,
    pub link: Option<String>,
    pub content: String,
    pub date: DateTime<Utc>,
}

pub(crate) fn parse_date(date_str: &str) -> DateTime<Utc> {
    DateTime::parse_from_rfc3339(date_str)
        .or_else(|_| DateTime::parse_from_rfc2822(date_str))
        .map(|dt| dt.with_timezone(&Utc))
        .unwrap_or_else(|_| Utc::now())
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
