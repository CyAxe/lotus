use mlua::UserData;
use reqwest::header::HeaderMap;
use reqwest::header::{HeaderName, HeaderValue};

#[derive(Clone)]
pub struct Sender {
    pub headers: HeaderMap,
    pub proxy: Option<String>,
    pub timeout: u64,
    pub redirects: u32,
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

        methods.add_method_mut("set_redirects", |_, this, redirects: u32| {
            this.redirects = redirects;
            Ok(())
        });
        methods.add_async_method(
            "send",
            |_, this, (method, url, req_body, req_headers): (String, String, mlua::Value, mlua::Value)| async move {
                
                let mut all_headers = HeaderMap::new();
                match req_headers {
                    mlua::Value::Table(current_headers) => {
                        current_headers.pairs::<String,String>().for_each(|currentheader| {
                            all_headers.insert(HeaderName::from_bytes(currentheader.clone().unwrap().0.as_bytes()).unwrap(), HeaderValue::from_bytes(currentheader.unwrap().1.as_bytes()).unwrap());
                        });
                    },
                    _ => {
                    }
                };
                let body: String = match req_body {
                    mlua::Value::String(body) => {
                        body.to_str().unwrap().to_string()
                    },
                    _ => {
                        "".to_string()
                    }
                };

                let resp = this.send(&method, url, body, all_headers).await;
                Ok(resp)
            },
        );
    }
}
