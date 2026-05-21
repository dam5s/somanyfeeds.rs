use std::time::Duration;
use tokio::time::interval;
use std::sync::Arc;
use crate::feeds::{FeedRecord, FeedsRepository};
use crate::articles::{ArticleRecord, ArticlesRepository};
use feeds_processing::FeedsProcessingError;

pub struct WorkerSettings {
    pub interval_seconds: u64,
}

impl WorkerSettings {
    pub fn new(interval_seconds: u64) -> Self {
        let interval_seconds = if interval_seconds == 0 { 30 } else { interval_seconds };
        Self { interval_seconds }
    }
}

pub struct Worker {
    settings: WorkerSettings,
    feeds_repository: Arc<FeedsRepository>,
    articles_repository: Arc<ArticlesRepository>,
}

impl Worker {
    pub fn new(
        settings: WorkerSettings,
        feeds_repository: FeedsRepository,
        articles_repository: ArticlesRepository,
    ) -> Self {
        Self {
            settings,
            feeds_repository: Arc::new(feeds_repository),
            articles_repository: Arc::new(articles_repository),
        }
    }

    pub fn start(&self) {
        let interval_seconds = self.settings.interval_seconds;
        let feeds_repository = self.feeds_repository.clone();
        let articles_repository = self.articles_repository.clone();

        tokio::spawn(async move {
            let mut interval = interval(Duration::from_secs(interval_seconds));
            loop {
                interval.tick().await;
                Self::run_work(feeds_repository.clone(), articles_repository.clone()).await;
            }
        });
    }

    pub async fn run_work(
        feeds_repository: Arc<FeedsRepository>,
        articles_repository: Arc<ArticlesRepository>,
    ) {
        println!("Worker is running...");
        let feeds = feeds_repository.find_all().await;
        let mut all_articles = Vec::new();

        for feed in feeds {
            println!("Loading feed: {} ({})", feed.name, feed.url);
            match Self::fetch_feed_articles(&feed).await {
                Ok(articles) => {
                    println!("Successfully fetched {} articles from {}", articles.len(), feed.name);
                    all_articles.extend(articles);
                }
                Err(e) => match e {
                    FeedsProcessingError::Network(_) => eprintln!("Error downloading feed {}: {:?}", feed.url, e),
                    FeedsProcessingError::Parsing(_) => eprintln!("Error parsing feed {}: {:?}", feed.url, e),
                },
            }
        }

        let article_count = all_articles.len();
        articles_repository.replace_all(all_articles).await;
        println!("Worker run finished. Total articles: {}", article_count);
    }

    async fn fetch_feed_articles(feed: &FeedRecord) -> Result<Vec<ArticleRecord>, FeedsProcessingError> {
        let downloaded = feeds_processing::download_content(&feed.url).await?;
        let articles = feeds_processing::parse_feed(&downloaded.content)?;

        Ok(articles
            .into_iter()
            .map(|article| ArticleRecord {
                title: article.title,
                link: article.link,
                content: article.content,
                date: article.date,
                feed_name: feed.name.clone(),
                feed_url: feed.url.clone(),
            })
            .collect())
    }
}
