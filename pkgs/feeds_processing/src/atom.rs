use serde::Deserialize;
use crate::{Article, FeedsProcessingError};

#[derive(Deserialize)]
pub struct Atom {
    #[serde(rename = "entry", default)]
    pub entries: Vec<AtomEntry>,
}

#[derive(Deserialize)]
pub struct AtomEntry {
    pub title: Option<AtomTitle>,
}

#[derive(Deserialize)]
pub struct AtomTitle {
    #[serde(rename = "$value")]
    pub content: String,
}

pub fn parse(content: &str) -> Result<Vec<Article>, FeedsProcessingError> {
    let atom = quick_xml::de::from_str::<Atom>(content)
        .map_err(|e| FeedsProcessingError::Parsing(e.to_string()))?;

    Ok(atom
        .entries
        .into_iter()
        .map(|entry| Article {
            title: entry.title.map(|t| t.content),
        })
        .collect())
}
