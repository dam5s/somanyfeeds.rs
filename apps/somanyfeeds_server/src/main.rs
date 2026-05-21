use std::sync::Arc;
use somanyfeeds_server::{
    app,
    env::load_env_num,
    feeds::{FeedRecord, FeedsRepository},
    articles::ArticlesRepository,
    worker::{Worker, WorkerSettings},
};

#[tokio::main]
async fn main() {
    let worker_interval_seconds = load_env_num("WORKER_INTERVAL_SECONDS", 300);
    let worker_settings = WorkerSettings::new(worker_interval_seconds);

    let feeds = vec![
        FeedRecord {
            name: "Github".to_string(),
            url: "https://github.com/dam5s.atom".to_string(),
        },
        FeedRecord {
            name: "Blog".to_string(),
            url: "https://blog.damo.io/rss.xml".to_string(),
        },
        FeedRecord {
            name: "Mastodon".to_string(),
            url: "https://mastodon.kleph.eu/users/dam5s.rss".to_string(),
        },
        FeedRecord {
            name: "Bluesky".to_string(),
            url: "https://bsky.app/profile/did:plc:zvnvcicnso363xz3gu6ho3mw/rss".to_string(),
        },
    ];
    let feeds_repository = Arc::new(FeedsRepository::new(feeds));
    let articles_repository = Arc::new(ArticlesRepository::default());

    Worker::new(worker_settings, feeds_repository, articles_repository.clone()).start();

    let port: u16 = load_env_num("PORT", 3000);
    let server_addr = format!("127.0.0.1:{}", port);

    let listener = tokio::net::TcpListener::bind(&server_addr).await.unwrap();

    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app(articles_repository)).await.unwrap();
}
