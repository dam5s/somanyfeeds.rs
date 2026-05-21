use somanyfeeds_server::worker::Worker;
use somanyfeeds_server::feeds::{FeedRecord, FeedsRepository};
use somanyfeeds_server::articles::ArticlesRepository;
use std::sync::Arc;
use wiremock::{MockServer, Mock, ResponseTemplate};
use wiremock::matchers::{method, path};

#[tokio::test]
async fn test_worker_run_work_limits_to_20_most_recent_articles() {
    let mock_server = MockServer::start().await;
    
    let mut feed_content = String::from(r#"<?xml version="1.0" encoding="utf-8"?>
<rss version="2.0">
    <channel>
        <title>Test Feed</title>"#);

    let days_of_week = ["Fri", "Sat", "Sun", "Mon", "Tue", "Wed", "Thu"];
    for i in 1..=25 {
        // Date: higher i means more recent
        let day = format!("{:02}", i);
        let dow = days_of_week[(i - 1) % 7];
        feed_content.push_str(&format!(r#"
        <item>
            <title>Article {i}</title>
            <link>https://example.com/{i}</link>
            <description>Content {i}</description>
            <pubDate>{}, {} May 2026 12:00:00 +0000</pubDate>
        </item>"#, dow, day));
    }

    feed_content.push_str(r#"
    </channel>
</rss>"#);

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
    
    // Should be limited to 20
    assert_eq!(articles.len(), 20);
    
    // Should be the most recent ones (Article 25 down to Article 6)
    // We expect them to be sorted by date descending if the implementation sorts them that way,
    // or at least be the 20 most recent ones.
    
    let mut article_titles: Vec<String> = articles.iter().map(|a| a.title.clone().unwrap()).collect();
    article_titles.sort(); // Sort titles to easily check the range
    
    for i in 6..=25 {
        let expected_title = format!("Article {}", i);
        assert!(article_titles.contains(&expected_title), "Missing {}", expected_title);
    }
    
    for i in 1..6 {
        let unexpected_title = format!("Article {}", i);
        assert!(!article_titles.contains(&unexpected_title), "Should not have {}", unexpected_title);
    }
}
