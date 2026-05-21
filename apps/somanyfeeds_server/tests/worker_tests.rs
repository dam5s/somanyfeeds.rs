use somanyfeeds_server::worker::{Worker, WorkerSettings};
use somanyfeeds_server::feeds::{FeedRecord, FeedsRepository};
use somanyfeeds_server::articles::ArticlesRepository;
use std::sync::Arc;
use wiremock::{MockServer, Mock, ResponseTemplate};
use wiremock::matchers::{method, path};

#[test]
fn test_worker_settings_new() {
    let settings = WorkerSettings::new(60);
    assert_eq!(settings.interval_seconds, 60);
}

#[test]
fn test_worker_settings_new_zero_defaults_to_30() {
    let settings = WorkerSettings::new(0);
    assert_eq!(settings.interval_seconds, 30);
}

#[tokio::test]
async fn test_worker_run_work_accumulates_articles() {
    let mock_server = MockServer::start().await;
    
    let feed_content = r#"<?xml version="1.0" encoding="utf-8"?>
<rss version="2.0">
    <channel>
        <title>Test Feed</title>
        <item>
            <title>Article 1</title>
            <link>https://example.com/1</link>
            <description>Content 1</description>
            <pubDate>Mon, 01 Jan 2024 00:00:00 +0000</pubDate>
        </item>
    </channel>
</rss>"#;

    Mock::given(method("GET"))
        .and(path("/rss.xml"))
        .respond_with(ResponseTemplate::new(200).set_body_string(feed_content))
        .mount(&mock_server)
        .await;

    let feeds = vec![
        FeedRecord {
            name: "Test Feed".to_string(),
            url: format!("{}/rss.xml", mock_server.uri()),
        }
    ];
    let feeds_repository = Arc::new(FeedsRepository::new(feeds));
    let articles_repository = Arc::new(ArticlesRepository::default());

    Worker::run_work(feeds_repository, articles_repository.clone()).await;

    let articles = articles_repository.find_all().await;
    assert_eq!(articles.len(), 1);
    assert_eq!(articles[0].title, Some("Article 1".to_string()));
    assert_eq!(articles[0].feed_name, "Test Feed");
}

#[tokio::test]
async fn test_worker_new() {
    let settings = WorkerSettings::new(30);
    let feeds_repository = Arc::new(FeedsRepository::new(vec![]));
    let articles_repository = Arc::new(ArticlesRepository::default());
    let _worker = Worker::new(settings, feeds_repository, articles_repository);
}
