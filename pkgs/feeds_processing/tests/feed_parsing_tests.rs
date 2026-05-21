use feeds_processing::parse_feed;
use std::fs;
use std::path::PathBuf;
use chrono::{TimeZone, Utc};

#[test]
fn test_parse_rss_damo_io() {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("resources/test_samples/damo.io.xml");
    
    let content = fs::read_to_string(path).expect("Failed to read test file");
    let articles = parse_feed(&content).unwrap();

    // damo.io.xml has 5 items
    assert_eq!(articles.len(), 5);
    
    assert_eq!(articles[0].title, Some("Things to learn in React and Redux".to_string()));
    assert_eq!(articles[0].link, Some("https://blog.damo.io/posts/things-to-learn-in-react-redux".to_string()));
    assert!(articles[0].content.starts_with("<p>There is a lot of \"tutorials\" out there teaching React and Redux."));
    assert_eq!(articles[0].date, Utc.with_ymd_and_hms(2022, 2, 20, 21, 52, 0).unwrap());

    assert_eq!(articles[1].title, Some("initialMonitor".to_string()));
    assert_eq!(articles[2].title, Some("Error handling in Kotlin and any modern static type system".to_string()));
    assert_eq!(articles[3].title, Some("Testing Kotlin with a custom DSL for Aspen".to_string()));
    assert_eq!(articles[4].title, Some("Kotlin testing with Aspen and Aspen Spring".to_string()));
}

#[test]
fn test_parse_atom_github() {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("resources/test_samples/github.xml");
    
    let content = fs::read_to_string(path).expect("Failed to read test file");
    let articles = parse_feed(&content).unwrap();

    // github.xml has 7 entries
    assert_eq!(articles.len(), 7);
    assert_eq!(articles[0].title, Some("dam5s pushed to master in dam5s/somanyfeeds.fs".to_string()));
    assert_eq!(articles[0].link, Some("https://github.com/dam5s/somanyfeeds.fs".to_string()));
    assert_eq!(articles[0].content, "<p>Hello from the content</p>");
    assert_eq!(articles[0].date, Utc.with_ymd_and_hms(2018, 4, 14, 21, 30, 17).unwrap());
}

#[test]
fn test_parse_rss_mastodon() {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("resources/test_samples/mastodon.xml");
    
    let content = fs::read_to_string(path).expect("Failed to read test file");
    let articles = parse_feed(&content).unwrap();

    // mastodon.xml has 19 items
    assert_eq!(articles.len(), 19);
    
    assert_eq!(articles[0].title, None);
    assert_eq!(articles[0].link, Some("https://mastodon.kleph.eu/@dam5s/109791647108368266".to_string()));
    assert!(articles[0].content.starts_with("<p>As a programmer that writes code in many different languages"));
    assert_eq!(articles[0].date, Utc.with_ymd_and_hms(2023, 2, 1, 21, 39, 44).unwrap());

    // In this specific mastodon.xml, items don't have titles
    for article in &articles {
        assert_eq!(article.title, None);
    }
}

#[test]
fn test_parse_rss_bluesky() {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("resources/test_samples/bluesky.xml");
    
    let content = fs::read_to_string(path).expect("Failed to read test file");
    let articles = parse_feed(&content).unwrap();

    // bluesky.xml has 14 items
    assert_eq!(articles.len(), 14);
    
    assert_eq!(articles[0].title, None);
    assert_eq!(articles[0].link, Some("https://bsky.app/profile/damo.io/post/3mmaujfelw22s".to_string()));
    assert!(articles[0].content.starts_with("@duckduckgo.com and @kagi.com are excellent alternatives!"));
    assert_eq!(articles[0].date, Utc.with_ymd_and_hms(2026, 5, 20, 2, 33, 0).unwrap());

    // In this specific bluesky.xml, items don't have titles
    for article in &articles {
        assert_eq!(article.title, None);
    }
}
