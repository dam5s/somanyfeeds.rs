use somanyfeeds_server::articles::{ArticleRecord, ArticlesRepository};
use chrono::Utc;

#[tokio::test]
async fn it_replaces_and_finds_all_articles() {
    let repository = ArticlesRepository::new(Vec::new());

    let articles = vec![
        ArticleRecord {
            title: Some("Article 1".to_string()),
            link: Some("https://example.com/1".to_string()),
            content: "Content 1".to_string(),
            date: Utc::now(),
            feed_name: "Feed 1".to_string(),
            feed_url: "https://feed1.com".to_string(),
        },
        ArticleRecord {
            title: Some("Article 2".to_string()),
            link: Some("https://example.com/2".to_string()),
            content: "Content 2".to_string(),
            date: Utc::now(),
            feed_name: "Feed 2".to_string(),
            feed_url: "https://feed2.com".to_string(),
        },
    ];

    repository.replace_all(articles.clone()).await;

    let result = repository.find_all().await;

    assert_eq!(result, articles);
}
