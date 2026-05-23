pub mod downloads;
pub use downloads::{DownloadedContent, download_content};

pub mod error;
pub use error::FeedsProcessingError;

pub mod feed_parsing;
pub use feed_parsing::{Article, parse_feed};

pub(crate) mod atom;
pub(crate) mod rss;
