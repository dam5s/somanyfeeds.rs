use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use tower::ServiceExt; // for `oneshot`
use somanyfeeds_server::routes::{router, RouterSettings};
use somanyfeeds_server::articles::{ArticleRecord, ArticlesRepository};
use std::sync::Arc;

#[tokio::test]
async fn test_articles_index() {
    let articles_repository = Arc::new(ArticlesRepository::default());
    
    // Use a specific date to test formatting (Denver is UTC-6 in May)
    let date_str = "2026-05-22T05:35:00Z";
    let base_date = chrono::DateTime::parse_from_rfc3339(date_str).unwrap().with_timezone(&chrono::Utc);
    
    let mut articles = Vec::new();
    
    // Article with specific content and date for formatting check
    articles.push(ArticleRecord {
        title: Some("Article 1".to_string()),
        content: "Content 1".to_string(),
        date: base_date,
        ..ArticleRecord::default()
    });

    // Add more articles to test sorting
    for i in 2..=40 {
        articles.push(ArticleRecord {
            title: Some(format!("Article {}", i)),
            // Higher i means more recent
            date: base_date + chrono::TimeDelta::try_seconds(i as i64).unwrap(),
            ..ArticleRecord::default()
        });
    }
    
    articles_repository.replace_all(articles).await;

    let settings = RouterSettings {
        public_path: format!("{}/resources/public", env!("CARGO_MANIFEST_DIR")),
    };
    let app = router(articles_repository, settings);

    let response = app
        .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let body_str = String::from_utf8(body.to_vec()).unwrap();
    
    // Verify content
    assert!(body_str.contains("Article 1"));
    assert!(body_str.contains("Content 1"));
    
    // Verify date formatting
    assert!(body_str.contains("May 21 '26 @ 23:35"));
    
    // Verify sorting (Article 40 should be before Article 1)
    assert!(body_str.contains("Article 40"));
    let pos_40 = body_str.find("Article 40").expect("Article 40 not found");
    let pos_1 = body_str.find("Article 1").expect("Article 1 not found");
    assert!(pos_40 < pos_1, "Article 40 should appear before Article 1 (sorted by date descending)");
}

#[tokio::test]
async fn test_static_assets() {
    let articles_repository = Arc::new(ArticlesRepository::default());
    let settings = RouterSettings {
        public_path: format!("{}/resources/public", env!("CARGO_MANIFEST_DIR")),
    };
    let app = router(articles_repository, settings);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/app.css")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let body_str = String::from_utf8(body.to_vec()).unwrap();
    assert!(body_str.contains(":root {"));
}
