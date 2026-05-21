use std::fmt;

pub fn process_feeds() {
    println!("Processing feeds...");
}

pub struct DownloadedContent {
    pub content: String,
}

#[derive(Debug)]
pub enum DownloadError {
    Network(String),
}

impl fmt::Display for DownloadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DownloadError::Network(msg) => write!(f, "Network error: {}", msg),
        }
    }
}

impl std::error::Error for DownloadError {}

pub async fn download_content(url: &str) -> Result<DownloadedContent, DownloadError> {
    let response = reqwest::get(url).await.map_err(|e| DownloadError::Network(e.to_string()))?;
    let content = response.text().await.map_err(|e| DownloadError::Network(e.to_string()))?;
    Ok(DownloadedContent { content })
}
