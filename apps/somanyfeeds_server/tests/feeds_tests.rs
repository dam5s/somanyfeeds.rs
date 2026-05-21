use somanyfeeds_server::feeds::{FeedRecord, FeedsRepository};

#[tokio::test]
async fn it_finds_all_feeds() {
    let feeds = vec![
        FeedRecord {
            name: "Feed 1".to_string(),
            url: "https://feed1.com".to_string(),
        },
        FeedRecord {
            name: "Feed 2".to_string(),
            url: "https://feed2.com".to_string(),
        },
    ];
    let repository = FeedsRepository::new(feeds.clone());

    let result = repository.find_all().await;

    assert_eq!(result, feeds);
}
