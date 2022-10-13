use isahc::{
    config::{RedirectPolicy, SslOption},
    prelude::*,
    HttpClientBuilder, Request,
};
use mlua::UserData;
use std::{collections::HashMap, time::Duration};
use tealr::{mlu::FromToLua, TypeName};

#[derive(Clone)]
pub struct Sender {
    proxy: Option<url::Url>,
    timeout: u64,
    redirects: u32,
}

#[derive(FromToLua, Clone, Debug, TypeName)]
pub enum RespType {
    NoErrors,
    Emtpy,
    Str(String),
    Int(i32),
    Headers(HashMap<String, String>),
    Error(String),
}

impl UserData for Sender {
    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_async_method(
            "send",
            |_, this, (method, url, body): (String, String, String)| async move {
                let resp = this.send(&method, url, body).await;
                Ok(resp)
            },
        );
    }
}

impl Sender {
    pub fn init(proxy: Option<url::Url>, timeout: u64, redirects: u32) -> Sender {
        Sender {
            timeout,
            redirects,
            proxy,
        }
    }
    fn build_client(&self) -> Result<isahc::HttpClient, isahc::Error> {
        HttpClientBuilder::new()
            .timeout(Duration::from_secs(self.timeout))
            .redirect_policy(RedirectPolicy::Limit(self.redirects))
            .proxy(None)
            .ssl_options(SslOption::DANGER_ACCEPT_INVALID_CERTS)
            .ssl_options(SslOption::DANGER_ACCEPT_REVOKED_CERTS)
            .ssl_options(SslOption::DANGER_ACCEPT_INVALID_HOSTS)
            .build()
    }
    pub async fn send(&self, method: &str, url: String, body: String) -> HashMap<String, RespType> {
        let mut resp_data: HashMap<String, RespType> = HashMap::new();
        let current_request = Request::builder()
            .uri(url)
            .method(method)
            .body(body)
            .unwrap();
        let req = self
            .build_client()
            .unwrap()
            .send_async(current_request)
            .await;

        match req {
            Ok(mut resp) => {
                let mut resp_headers: HashMap<String, String> = HashMap::new();
                resp.headers()
                    .iter()
                    .for_each(|(header_name, header_value)| {
                        resp_headers.insert(
                            header_name.to_string(),
                            header_value.to_str().unwrap().to_string(),
                        );
                    });
                resp_data.insert(
                    "url".to_string(),
                    RespType::Str(resp.effective_uri().unwrap().to_string()),
                );
                resp_data.insert(
                    "status".to_string(),
                    RespType::Str(resp.status().to_string()),
                );
                resp_data.insert(
                    "body".to_string(),
                    RespType::Str(resp.text().await.unwrap()),
                );
                resp_data.insert("errors".to_string(), RespType::NoErrors);
                resp_data.insert("headers".to_string(), RespType::Headers(resp_headers));
                resp_data
            }
            Err(err) => {
                resp_data.insert("status".to_string(), RespType::Emtpy);
                resp_data.insert("body".to_string(), RespType::Emtpy);
                resp_data.insert("errors".to_string(), RespType::Error(err.to_string()));
                resp_data.insert("headers".to_string(), RespType::Headers(HashMap::new()));
                resp_data
            }
        }
    }
}
