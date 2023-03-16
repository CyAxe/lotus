use lotus::lua::network::http::Sender;
use reqwest::header::{HeaderMap, HeaderValue};

#[tokio::test]
async fn test_send() {
    // Create test sender
    let sender = Sender::init(HeaderMap::new(), None, 10, 3);

    // Send test request
    let resp = sender
        .send(
            "GET",
            "https://httpbin.org/get".to_string(),
            None,
            sender.clone()
        )
        .await;
    assert!(resp.is_ok());

    // Check response body
    let resp_body = resp.unwrap().body;
    assert!(resp_body.contains(r#""url": "https://httpbin.org/get""#));
}

#[test]
fn test_init() {
    // Create test headers
    let headers = HeaderMap::new();

    // Create test sender
    let sender = Sender::init(
        headers,
        Some("http://proxy.example.com:8080".to_string()),
        10,
        3,
    );

    // Check sender properties
    assert_eq!(
        sender.proxy,
        Some("http://proxy.example.com:8080".to_string())
    );
    assert_eq!(sender.timeout, 10);
    assert_eq!(sender.redirects, 3);
}

#[test]
fn test_build_client() {
    // Create test headers
    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", HeaderValue::from_static("application/json"));

    // Create test sender
    let sender = Sender::init(
        headers.clone(),
        Some("http://proxy.example.com:8080".to_string()),
        10,
        3,
    );

    assert_eq!(
        sender.proxy.unwrap().as_str(),
        "http://proxy.example.com:8080"
    );

    // Test no-proxy client builder
    let sender = Sender::init(headers.clone(), None, 10, 3);
    assert!(sender.proxy.is_none());
}
