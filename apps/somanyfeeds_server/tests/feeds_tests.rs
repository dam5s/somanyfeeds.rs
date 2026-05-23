use somanyfeeds_server::feeds::{FeedRecord, FeedsRepository};

#[tokio::test]
async fn it_finds_all_feeds() {
    let feeds = vec![
        FeedRecord {
            name: "Feed 1".to_string(),
            ..FeedRecord::default()
        },
        FeedRecord {
            name: "Feed 2".to_string(),
            ..FeedRecord::default()
        },
    ];
    let repository = FeedsRepository::new(feeds.clone());

    let result = repository.find_all().await;

    assert_eq!(result, feeds);
}
