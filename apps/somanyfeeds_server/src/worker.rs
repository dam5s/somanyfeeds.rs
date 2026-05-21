use std::time::Duration;
use tokio::time::interval;
use std::sync::Arc;
use crate::feeds::FeedsRepository;

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
}

impl Worker {
    pub fn new(settings: WorkerSettings, repository: FeedsRepository) -> Self {
        Self {
            settings,
            feeds_repository: Arc::new(repository),
        }
    }

    pub fn start(&self) {
        let interval_seconds = self.settings.interval_seconds;
        let repository = self.feeds_repository.clone();

        tokio::spawn(async move {
            let mut interval = interval(Duration::from_secs(interval_seconds));
            loop {
                interval.tick().await;
                Self::run_work(repository.clone()).await;
            }
        });
    }

    async fn run_work(repository: Arc<FeedsRepository>) {
        println!("Worker is running...");
        let feeds = repository.find_all().await;
        for feed in feeds {
            println!("Loading feed: {} ({})", feed.name, feed.url);
        }
    }
}
