use feeds_processing::parse_feed;
use std::fs;
use std::path::PathBuf;

#[test]
fn test_parse_rss_damo_io() {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("resources/test_samples/damo.io.xml");
    
    let content = fs::read_to_string(path).expect("Failed to read test file");
    let articles = parse_feed(&content).unwrap();

    // damo.io.xml has 5 items
    assert_eq!(articles.len(), 5);
    
    assert_eq!(articles[0].title, "Things to learn in React and Redux");
    assert_eq!(articles[1].title, "initialMonitor");
    assert_eq!(articles[2].title, "Error handling in Kotlin and any modern static type system");
    assert_eq!(articles[3].title, "Testing Kotlin with a custom DSL for Aspen");
    assert_eq!(articles[4].title, "Kotlin testing with Aspen and Aspen Spring");
}

#[test]
fn test_parse_atom_github() {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("resources/test_samples/github.xml");
    
    let content = fs::read_to_string(path).expect("Failed to read test file");
    let articles = parse_feed(&content).unwrap();

    // github.xml has 7 entries
    assert_eq!(articles.len(), 7);
    assert_eq!(articles[0].title, "dam5s pushed to master in dam5s/somanyfeeds.fs");
}

#[test]
fn test_parse_rss_mastodon() {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("resources/test_samples/mastodon.xml");
    
    let content = fs::read_to_string(path).expect("Failed to read test file");
    let articles = parse_feed(&content).unwrap();

    // mastodon.xml has 19 items
    assert_eq!(articles.len(), 19);
    
    // In this specific mastodon.xml, items don't have titles
    for article in articles {
        assert_eq!(article.title, "");
    }
}
