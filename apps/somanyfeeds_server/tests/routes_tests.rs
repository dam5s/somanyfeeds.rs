use axum::{
    Router,
    body::Body,
    http::{Request, StatusCode},
};
use somanyfeeds_server::articles::{ArticleRecord, ArticlesRepository};
use somanyfeeds_server::routes::{RouterSettings, router};
use std::sync::Arc;
use tower::ServiceExt; // for `oneshot`

fn app(articles: Vec<ArticleRecord>) -> Router {
    let articles_repository = Arc::new(ArticlesRepository::new(articles));
    let settings = RouterSettings {
        public_path: format!("{}/resources/public", env!("CARGO_MANIFEST_DIR")),
    };
    router(articles_repository, settings)
}

#[tokio::test]
async fn test_articles_index() {
    // Use a specific date to test formatting (Denver is UTC-6 in May)
    let date_str = "2026-05-22T05:35:00Z";
    let base_date = chrono::DateTime::parse_from_rfc3339(date_str)
        .unwrap()
        .with_timezone(&chrono::Utc);

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

    let app = app(articles);

    let response = app
        .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
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
    assert!(
        pos_40 < pos_1,
        "Article 40 should appear before Article 1 (sorted by date descending)"
    );

    // Verify "About" article is present and prepended
    assert!(body_str.contains("<article class=\"About\">"));
    assert!(body_str.contains("Damien Le Berrigaud"));
    let pos_about = body_str
        .find("<article class=\"About\">")
        .expect("About article not found");
    assert!(
        pos_about < pos_40,
        "About article should appear before Article 40"
    );
}

#[tokio::test]
async fn test_article_without_title_with_link() {
    let link = "https://example.com/some-article";

    let app = app(vec![ArticleRecord {
        title: None,
        link: Some(link.to_string()),
        date: chrono::Utc::now(),
        ..ArticleRecord::default()
    }]);

    let response = app
        .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
        .await
        .unwrap();

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let body_str = String::from_utf8(body.to_vec()).unwrap();

    // Verify "Source" link is present in a nav
    assert!(
        body_str.contains("<nav>\n                <a href=\"https://example.com/some-article\">Source</a>\n            </nav>"),
        "Source link should be present in a nav when title is missing. Body: {}", body_str
    );

    // Verify date is NOT linked
    assert!(
        !body_str.contains(&format!("<h2>\n            \n                <a href=\"{}\">", link)),
        "Date should not be linked. Body: {}", body_str
    );
}

#[tokio::test]
async fn test_article_with_title_and_link() {
    let link = "https://example.com/some-article";
    let title = "Article Title";

    let app = app(vec![ArticleRecord {
        title: Some(title.to_string()),
        link: Some(link.to_string()),
        date: chrono::Utc::now(),
        ..ArticleRecord::default()
    }]);

    let response = app
        .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
        .await
        .unwrap();

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let body_str = String::from_utf8(body.to_vec()).unwrap();

    // Verify link is present on title
    assert!(
        body_str.contains(&format!("<h1>\n                \n                    <a href=\"{}\">{}</a>", link, title)),
        "Link should be present on title. Body: {}", body_str
    );

    // Verify link is NOT present on date
    assert!(
        !body_str.contains(&format!("<h2>\n            \n                <a href=\"{}\">", link)),
        "Link should NOT be present on date. Body: {}", body_str
    );
}

#[tokio::test]
async fn test_static_assets() {
    let app = app(vec![]);

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
