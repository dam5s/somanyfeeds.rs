use somanyfeeds_server::{
    app,
    env::load_env_num,
    feeds::{FeedRecord, FeedsRepository},
    articles::ArticlesRepository,
    worker::{Worker, WorkerSettings},
};

#[tokio::main]
async fn main() {
    let worker_interval_seconds = load_env_num("WORKER_INTERVAL_SECONDS", 30);
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
    let feeds_repository = FeedsRepository::new(feeds);
    let articles_repository = ArticlesRepository::default();

    Worker::new(worker_settings, feeds_repository, articles_repository).start();

    let port: u16 = load_env_num("PORT", 3000);
    let server_addr = format!("127.0.0.1:{}", port);

    let listener = tokio::net::TcpListener::bind(&server_addr).await.unwrap();

    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app()).await.unwrap();
}
