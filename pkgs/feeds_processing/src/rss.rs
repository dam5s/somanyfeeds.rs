use crate::{Article, FeedsProcessingError};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Rss {
    pub channel: RssChannel,
}

#[derive(Deserialize)]
pub struct RssChannel {
    #[serde(rename = "item", default)]
    pub items: Vec<RssItem>,
}

#[derive(Deserialize)]
pub struct RssItem {
    pub title: Option<String>,
    pub link: Option<String>,
    pub description: Option<String>,
    #[serde(rename = "pubDate")]
    pub pub_date: Option<String>,
}

pub fn parse(content: &str) -> Result<Vec<Article>, FeedsProcessingError> {
    let rss = quick_xml::de::from_str::<Rss>(content)
        .map_err(|e| FeedsProcessingError::Parsing(e.to_string()))?;

    Ok(rss
        .channel
        .items
        .into_iter()
        .map(|item| Article {
            title: item.title,
            link: item.link,
            content: item.description.unwrap_or_default(),
            date: item
                .pub_date
                .map(|d| crate::feed_parsing::parse_date(&d))
                .unwrap_or_else(chrono::Utc::now),
        })
        .collect())
}
