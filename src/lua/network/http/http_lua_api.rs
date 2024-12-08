use mlua::UserData;
use reqwest::header::HeaderMap;
use reqwest::header::{HeaderName, HeaderValue};
use std::collections::HashMap;
use tealr::{mlu::FromToLua, TypeName};

#[derive(Clone)]
pub struct Sender {
    pub headers: HeaderMap, // HTTP headers to include in requests.
    pub proxy: Option<String>, // Optional proxy setting for requests.
    pub timeout: u64, // Request timeout in seconds.
    pub redirects: u32, // Maximum number of allowed redirects.
    pub merge_headers: bool, // Whether to merge custom headers with default headers.
    pub http_options: HttpVersion, // HTTP version options.
}

#[derive(Clone)]
pub struct HttpVersion {
    pub http2_only: bool, // Flag to force HTTP/2 only.
    pub http1_only: bool, // Flag to force HTTP/1.1 only.
}

#[derive(TypeName, FromToLua)]
pub struct MultiPart {
    pub content: String, // Content of the multipart segment.
    pub filename: Option<String>, // Optional filename for the content.
    pub content_type: Option<String>, // Optional MIME type of the content.
    pub headers: Option<HashMap<String, String>>, // Additional headers for the multipart segment.
}

impl Default for HttpVersion {
    fn default() -> Self {
        Self {
            http1_only: false,
            http2_only: false,
        }
    }
}

// Implements additional methods for the `Sender` struct to enhance functionality.
impl UserData for Sender {
    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        // Sets the proxy for the sender.
        methods.add_method_mut("set_proxy", |_, this, the_proxy: mlua::Value| {
            match the_proxy {
                mlua::Value::String(new_proxy) => {
                    this.proxy = Some(new_proxy.to_str().unwrap().to_string());
                }
                _ => {
                    this.proxy = None;
                }
            };
            Ok(())
        });

        // Updates the timeout value.
        methods.add_method_mut("set_timeout", |_, this, timeout: u64| {
            this.timeout = timeout;
            Ok(())
        });

        // Toggles header merging behavior.
        methods.add_method_mut("merge_headers", |_, this, merge_headers: bool| {
            this.merge_headers = merge_headers;
            Ok(())
        });

        // Sets the maximum number of redirects.
        methods.add_method_mut("set_redirects", |_, this, redirects: u32| {
            this.redirects = redirects;
            Ok(())
        });

        // Asynchronously sends an HTTP request based on provided options.
        methods.add_async_method("send", |_, this, request_option: mlua::Table| async move {
            let url = request_option.get::<_, String>("url");
            if url.is_err() {
                return Err(mlua::Error::RuntimeError(
                    "URL Parameter is required".to_string(),
                ));
            }
            let url = url.unwrap();

            let multipart = request_option
                .get::<_, Option<HashMap<String, MultiPart>>>("multipart")
                .unwrap_or_default();
            let method = request_option
                .get::<_, Option<String>>("method")
                .ok()
                .flatten()
                .unwrap_or_else(|| "GET".to_string());
            let http2_only = request_option
                .get::<_, Option<bool>>("http2_only")
                .ok()
                .flatten()
                .unwrap_or_else(|| false);
            let http1_only = request_option
                .get::<_, Option<bool>>("http1_only")
                .ok()
                .flatten()
                .unwrap_or_else(|| false);
            let timeout = request_option
                .get::<_, Option<u64>>("timeout")
                .ok()
                .flatten()
                .unwrap_or(this.timeout);
            let proxy = request_option
                .get::<_, Option<String>>("proxy")
                .ok()
                .flatten()
                .or_else(|| this.proxy.clone());
            let redirects = request_option
                .get::<_, Option<u32>>("redirect")
                .ok()
                .flatten()
                .unwrap_or(this.redirects);
            let headers = match request_option.get::<_, Option<HashMap<String, String>>>("headers") {
                Ok(Some(headers)) => {
                    let mut current_headers = HeaderMap::new();
                    headers.iter().for_each(|(name, value)| {
                        current_headers.insert(
                            HeaderName::from_bytes(name.as_bytes()).unwrap(),
                            HeaderValue::from_bytes(value.as_bytes()).unwrap(),
                        );
                    });
                    if this.merge_headers {
                        current_headers.extend(this.headers.clone());
                    }
                    current_headers
                }
                _ => this.headers.clone(),
            };
            let body = match request_option.get::<_, Option<String>>("body") {
                Ok(Some(body)) if !body.is_empty() => Some(body),
                _ => None,
            };
            let current_request = Sender {
                timeout,
                proxy,
                headers,
                redirects,
                merge_headers: true,
                http_options: HttpVersion {
                    http2_only,
                    http1_only,
                },
            };
            let resp = this
                .send(&method, url, body, multipart, current_request)
                .await;
            if let Ok(resp) = resp {
                Ok(resp)
            } else {
                Err(resp.unwrap_err())
            }
        });
    }
}
