use feeds_processing::download_content;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_download_content() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/test"))
        .respond_with(ResponseTemplate::new(200).set_body_string("hello world"))
        .mount(&mock_server)
        .await;

    let url = format!("{}/test", &mock_server.uri());
    let result = download_content(&url).await.ok().unwrap();

    assert_eq!(result.content, "hello world");
}

#[tokio::test]
async fn test_download_content_error() {
    let url = "http://non-existent-url.invalid";
    let result = download_content(&url).await;

    assert!(result.is_err());
    let error = result.err().unwrap();
    assert!(error.to_string().contains("Network error"));
}
