use lotus::lua::parsing::url::HttpMessage;
use std::collections::HashMap;
use url::Url;

#[test]
fn test_change_urlquery() {
    let url = Url::parse("https://example.com/path?key1=value1&key2=value2").unwrap();
    let http_msg = HttpMessage { url };
    let payload = "new_value";
    let expected_result = [("key1", "new_value"), ("key2", "new_value")]
        .iter()
        .map(|(k, v)| {
            (
                k.to_string(),
                format!("https://example.com/path?{}={}", k, v),
            )
        })
        .collect::<HashMap<String, String>>();

    let result = http_msg.change_urlquery(payload, true);
    assert_eq!(result, expected_result);
}

#[test]
fn test_set_urlvalue() {
    let url = Url::parse("https://example.com/path?key1=value1&key2=value2").unwrap();
    let http_msg = HttpMessage { url };
    let param = "key1";
    let payload = "new_value";
    let expected_result = "https://example.com/path?key2=value2&key1=value1new_value";

    let result = http_msg.set_urlvalue(param, payload);
    assert_eq!(result, expected_result);
}

#[test]
fn test_urljoin() {
    let url = Url::parse("https://example.com/path").unwrap();
    let http_msg = HttpMessage { url };
    let path = "subpath";
    let expected_result = "https://example.com/path/subpath";

    let result = http_msg.urljoin(path);
    assert_eq!(result, expected_result);
}
