use crate::FeedsProcessingError;

pub struct DownloadedContent {
    pub content: String,
}

pub async fn download_content(url: &str) -> Result<DownloadedContent, FeedsProcessingError> {
    let response = reqwest::get(url)
        .await
        .map_err(|e| FeedsProcessingError::Network(e.to_string()))?;
    let content = response
        .text()
        .await
        .map_err(|e| FeedsProcessingError::Network(e.to_string()))?;
    Ok(DownloadedContent { content })
}
