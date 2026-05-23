use crate::{Article, FeedsProcessingError};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Atom {
    #[serde(rename = "entry", default)]
    pub entries: Vec<AtomEntry>,
}

#[derive(Deserialize)]
pub struct AtomEntry {
    pub title: Option<AtomText>,
    #[serde(rename = "link", default)]
    pub links: Vec<AtomLink>,
    pub content: Option<AtomText>,
    pub summary: Option<AtomText>,
    pub published: Option<String>,
    pub updated: Option<String>,
}

#[derive(Deserialize)]
pub struct AtomText {
    #[serde(rename = "$value")]
    pub content: String,
}

#[derive(Deserialize)]
pub struct AtomLink {
    #[serde(rename = "@href")]
    pub href: String,
    #[serde(rename = "@rel")]
    pub rel: Option<String>,
}

pub fn parse(content: &str) -> Result<Vec<Article>, FeedsProcessingError> {
    let atom = quick_xml::de::from_str::<Atom>(content)
        .map_err(|e| FeedsProcessingError::Parsing(e.to_string()))?;

    Ok(atom
        .entries
        .into_iter()
        .map(|entry| {
            let title = entry.title.map(|t| t.content);
            let link = entry
                .links
                .iter()
                .find(|l| l.rel.as_deref() == Some("alternate"))
                .or_else(|| entry.links.first())
                .map(|l| l.href.clone());
            let content = entry
                .content
                .or(entry.summary)
                .map(|t| t.content)
                .unwrap_or_default();
            let date_str = entry.published.or(entry.updated);
            let date = date_str
                .map(|d| crate::feed_parsing::parse_date(&d))
                .unwrap_or_else(chrono::Utc::now);

            Article {
                title,
                link,
                content,
                date,
            }
        })
        .collect())
}
