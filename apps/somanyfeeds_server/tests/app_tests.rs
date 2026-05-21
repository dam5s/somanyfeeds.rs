use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use tower::ServiceExt; // for `oneshot`
use somanyfeeds_server::app::app;
use somanyfeeds_server::articles::{ArticleRecord, ArticlesRepository};
use std::sync::Arc;
use chrono::Utc;

#[tokio::test]
async fn it_lists_articles() {
    let articles_repository = Arc::new(ArticlesRepository::default());
    let now = Utc::now();
    let articles = vec![
        ArticleRecord {
            title: Some("Article 1".to_string()),
            link: Some("https://example.com/1".to_string()),
            content: "Content 1".to_string(),
            date: now,
            feed_name: "Feed 1".to_string(),
            feed_url: "https://feed1.com".to_string(),
        },
    ];
    articles_repository.replace_all(articles).await;

    let app = app(articles_repository);

    let response = app
        .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let body_str = String::from_utf8(body.to_vec()).unwrap();
    assert!(body_str.contains("Article 1"));
    assert!(body_str.contains("Content 1"));
}

#[tokio::test]
async fn it_sorts_and_limits_articles() {
    let articles_repository = Arc::new(ArticlesRepository::default());
    let now = Utc::now();
    let mut articles = Vec::new();
    for i in 0..40 {
        articles.push(ArticleRecord {
            title: Some(format!("Article {}", i)),
            link: None,
            content: format!("Content {}", i),
            date: now + chrono::TimeDelta::try_seconds(i as i64).unwrap(),
            feed_name: "Feed".to_string(),
            feed_url: "url".to_string(),
        });
    }
    articles_repository.replace_all(articles).await;

    let app = app(articles_repository);

    let response = app
        .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
        .await
        .unwrap();

    let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let body_str = String::from_utf8(body.to_vec()).unwrap();
    
    // Should contain Article 39 (newest)
    assert!(body_str.contains("Article 39"));
    // Should NOT contain Article 0 (oldest, beyond 30 limit)
    assert!(!body_str.contains("Article 0"));
}
