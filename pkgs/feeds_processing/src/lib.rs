pub mod downloads;
pub use downloads::{download_content, DownloadedContent};

pub mod error;
pub use error::FeedsProcessingError;

pub mod feed_parsing;
pub use feed_parsing::{Article, parse_feed};

pub(crate) mod rss;
pub(crate) mod atom;
