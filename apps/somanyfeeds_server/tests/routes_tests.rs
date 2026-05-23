use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use tower::ServiceExt; // for `oneshot`
use somanyfeeds_server::routes::{router, RouterSettings};
use somanyfeeds_server::articles::{ArticleRecord, ArticlesRepository};
use std::sync::Arc;
use chrono::Utc;

#[tokio::test]
async fn it_lists_articles() {
    let articles_repository = Arc::new(ArticlesRepository::default());
    let articles = vec![
        ArticleRecord {
            title: Some("Article 1".to_string()),
            content: "Content 1".to_string(),
            ..ArticleRecord::default()
        },
    ];
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
    assert!(body_str.contains("Article 1"));
    assert!(body_str.contains("Content 1"));
}

#[tokio::test]
async fn it_formats_the_date() {
    let articles_repository = Arc::new(ArticlesRepository::default());
    let date = chrono::DateTime::parse_from_rfc3339("2026-05-22T05:35:00Z").unwrap().with_timezone(&chrono::Utc);
    let articles = vec![
        ArticleRecord {
            date,
            ..ArticleRecord::default()
        },
    ];
    articles_repository.replace_all(articles).await;

    let settings = RouterSettings {
        public_path: format!("{}/resources/public", env!("CARGO_MANIFEST_DIR")),
    };
    let app = router(articles_repository, settings);

    let response = app
        .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
        .await
        .unwrap();

    let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let body_str = String::from_utf8(body.to_vec()).unwrap();
    
    assert!(body_str.contains("May 21 '26 @ 23:35"));
}

#[tokio::test]
async fn it_sorts_articles() {
    let articles_repository = Arc::new(ArticlesRepository::default());
    let now = Utc::now();
    let mut articles = Vec::new();
    for i in 0..40 {
        articles.push(ArticleRecord {
            title: Some(format!("Article {}", i)),
            date: now + chrono::TimeDelta::try_seconds(i as i64).unwrap(),
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

    let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let body_str = String::from_utf8(body.to_vec()).unwrap();
    
    // Should contain Article 39 (newest)
    assert!(body_str.contains("Article 39"));
    // Should ALSO contain Article 0 (oldest, no longer truncated)
    assert!(body_str.contains("Article 0"));
}

#[tokio::test]
async fn it_serves_static_files() {
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
