use mlua::UserData;
use reqwest::header::HeaderMap;
use reqwest::header::{HeaderName, HeaderValue};
use std::collections::HashMap;
use tealr::{mlu::FromToLua, TypeName};

#[derive(Clone)]
pub struct Sender {
    pub headers: HeaderMap,
    pub proxy: Option<String>,
    pub timeout: u64,
    pub redirects: u32,
    pub merge_headers: bool,
}

#[derive(TypeName, FromToLua)]
pub struct MultiPart {
    pub content: String,
    pub filename: Option<String>,
    pub content_type: Option<String>,
    pub headers: Option<HashMap<String, String>>,
}

/// Adding OOP for http sender class
impl UserData for Sender {
    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
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
        methods.add_method_mut("set_timeout", |_, this, timeout: u64| {
            this.timeout = timeout;
            Ok(())
        });

        methods.add_method_mut("merge_headers", |_, this, merge_headers: bool| {
            this.merge_headers = merge_headers;
            Ok(())
        });
        methods.add_method_mut("set_redirects", |_, this, redirects: u32| {
            this.redirects = redirects;
            Ok(())
        });
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
            let timeout = request_option
                .get::<_, Option<u64>>("timeout")
                .ok()
                .flatten()
                .unwrap_or_else(|| this.timeout);
            let proxy = request_option
                .get::<_, Option<String>>("proxy")
                .ok()
                .flatten()
                .or_else(|| this.proxy.clone());
            let redirects = request_option
                .get::<_, Option<u32>>("redirect")
                .ok()
                .flatten()
                .unwrap_or_else(|| this.redirects);
            let headers = match request_option.get::<_, Option<HashMap<String, String>>>("headers")
            {
                Ok(Some(headers)) => {
                    let mut current_headers = HeaderMap::new();
                    headers.iter().for_each(|(name, value)| {
                        current_headers.insert(
                            HeaderName::from_bytes(name.as_bytes()).unwrap(),
                            HeaderValue::from_bytes(value.as_bytes()).unwrap(),
                        );
                    });
                    if this.merge_headers == true {
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
            };
            let resp = this
                .send(&method, url, body, multipart, current_request)
                .await;
            if resp.is_ok() {
                Ok(resp.unwrap())
            } else {
                Err(resp.unwrap_err())
            }
        });
    }
}
