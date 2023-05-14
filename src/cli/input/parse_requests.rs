use mlua::UserData;
use reqwest::Url;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tealr::TypeName;

use crate::lua::parsing::url::HttpMessage;

#[derive(TypeName,Deserialize, Serialize)]
pub struct FullRequest {
    pub method: String,
    pub url: String,
    pub headers: HashMap<String, String>,
    pub body: String,
}

impl Default for FullRequest {
    fn default() -> Self {
        let headers = HashMap::new();
        Self {
            method: "GET".to_string(),
            url: "http://example.com".to_string(),
            headers,
            body: "".to_string(),
        }
    }
}

impl FullRequest {
    fn inject_payloads(&self, payload: &str, remove_content: bool) -> HashMap<String, String> {
        let mut injected_params = HashMap::new();
        let url_parser = HttpMessage {
            url: Some(Url::parse(&self.url).unwrap())
        };
        let iter_params = url_parser.change_urlquery(payload, remove_content);
        injected_params.extend(iter_params);
        injected_params
    }
}


impl UserData for FullRequest {
    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("set_url_param", |_, this, (payload ,remove_param_content) : (String, Option<bool>)| {
            let remove_content = remove_param_content.unwrap_or(false);
            let test_data = this.inject_payloads(&payload,remove_content);
            Ok(test_data)
        });
    }
}
