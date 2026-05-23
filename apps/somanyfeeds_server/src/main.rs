use somanyfeeds_server::{
    articles::ArticlesRepository,
    env::{load_env_num, load_env_str},
    feeds::{FeedRecord, FeedsRepository},
    routes::{RouterSettings, router},
    worker::{Worker, WorkerSettings},
};
use std::sync::Arc;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "somanyfeeds_server=info,feeds_processing=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

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

    Worker::new(
        worker_settings,
        feeds_repository,
        articles_repository.clone(),
    )
    .start();

    let port: u16 = load_env_num("PORT", 3000);
    let server_addr = format!("127.0.0.1:{}", port);

    let default_public_path = format!("{}/resources/public", env!("CARGO_MANIFEST_DIR"));
    let public_path = load_env_str("PUBLIC_PATH", default_public_path);
    let router_settings = RouterSettings { public_path };

    let listener = tokio::net::TcpListener::bind(&server_addr).await.unwrap();

    info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, router(articles_repository, router_settings))
        .await
        .unwrap();
}
