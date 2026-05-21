use std::fmt;

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
