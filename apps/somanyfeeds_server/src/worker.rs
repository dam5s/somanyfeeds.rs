use std::time::Duration;
use tokio::time::interval;

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
}

impl Worker {
    pub fn new(settings: WorkerSettings) -> Self {
        Self { settings }
    }

    pub fn start(&self) {
        let interval_seconds = self.settings.interval_seconds;

        tokio::spawn(async move {
            let mut interval = interval(Duration::from_secs(interval_seconds));
            loop {
                interval.tick().await;
                Self::run_work();
            }
        });
    }

    fn run_work() {
        println!("Worker is running...");
    }
}
