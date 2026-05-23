#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct FeedRecord {
    pub name: String,
    pub url: String,
}

pub struct FeedsRepository {
    feeds: Vec<FeedRecord>,
}

impl FeedsRepository {
    pub fn new(feeds: Vec<FeedRecord>) -> Self {
        Self { feeds }
    }

    pub async fn find_all(&self) -> Vec<FeedRecord> {
        self.feeds.clone()
    }
}
