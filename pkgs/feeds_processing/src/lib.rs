use std::fmt;
use serde::Deserialize;

pub struct DownloadedContent {
    pub content: String,
}

pub struct Article {
    pub title: String,
}

#[derive(Debug)]
pub enum FeedsProcessingError {
    Network(String),
    Parsing(String),
}

impl fmt::Display for FeedsProcessingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FeedsProcessingError::Network(msg) => write!(f, "Network error: {}", msg),
            FeedsProcessingError::Parsing(msg) => write!(f, "Parsing error: {}", msg),
        }
    }
}

impl std::error::Error for FeedsProcessingError {}

pub async fn download_content(url: &str) -> Result<DownloadedContent, FeedsProcessingError> {
    let response = reqwest::get(url).await.map_err(|e| FeedsProcessingError::Network(e.to_string()))?;
    let content = response.text().await.map_err(|e| FeedsProcessingError::Network(e.to_string()))?;
    Ok(DownloadedContent { content })
}

#[derive(Deserialize)]
struct Rss {
    channel: RssChannel,
}

#[derive(Deserialize)]
struct RssChannel {
    #[serde(rename = "item", default)]
    items: Vec<RssItem>,
}

#[derive(Deserialize)]
struct RssItem {
    title: Option<String>,
}

#[derive(Deserialize)]
struct Atom {
    #[serde(rename = "entry", default)]
    entries: Vec<AtomEntry>,
}

#[derive(Deserialize)]
struct AtomEntry {
    title: AtomTitle,
}

#[derive(Deserialize)]
struct AtomTitle {
    #[serde(rename = "$value")]
    content: String,
}

pub fn parse_feed(content: &str) -> Result<Vec<Article>, FeedsProcessingError> {
    if let Ok(rss) = quick_xml::de::from_str::<Rss>(content) {
        return Ok(rss
            .channel
            .items
            .into_iter()
            .map(|item| Article {
                title: item.title.unwrap_or_default(),
            })
            .collect());
    }

    if let Ok(atom) = quick_xml::de::from_str::<Atom>(content) {
        return Ok(atom
            .entries
            .into_iter()
            .map(|entry| Article {
                title: entry.title.content,
            })
            .collect());
    }

    Err(FeedsProcessingError::Parsing(
        "Failed to parse feed as RSS or Atom".to_string(),
    ))
}
