use somanyfeeds_server::articles::{ArticleRecord, ArticlesRepository};

#[tokio::test]
async fn test_articles_repository() {
    let repository = ArticlesRepository::new(Vec::new());

    let articles = vec![
        ArticleRecord {
            title: Some("Article 1".to_string()),
            ..ArticleRecord::default()
        },
        ArticleRecord {
            title: Some("Article 2".to_string()),
            ..ArticleRecord::default()
        },
    ];

    repository.replace_all(articles.clone()).await;

    let result = repository.find_all().await;

    assert_eq!(result, articles);
}
