use mlua::UserData;
use reqwest::Url;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tealr::TypeName;

use crate::lua::parsing::url::HttpMessage;

#[derive(TypeName, Deserialize, Serialize)]
pub struct FullRequest {
    pub method: String,
    pub url: String,
    pub headers: HashMap<String, String>,
    pub body: String,
}

pub enum InjectionLocation {
    Url,
    Path,
    Headers,
    Body,
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
    fn inject_headers(
        &mut self,
        payload: &str,
        remove_content: bool,
    ) -> HashMap<String, FullRequest> {
        let headers = &self.headers;
        let mut results: HashMap<String, FullRequest> = HashMap::new();
        headers.iter().for_each(|(headername, headervalue)| {
            let current_headers = &mut self.headers.clone();
            let mut current_results: HashMap<String, String> = HashMap::new();
            current_results.insert(headername.into(), {
                if remove_content {
                    format!("{}{}", headervalue, payload)
                } else {
                    format!("{}", payload)
                }
            });
            current_headers.extend(current_results);
            results.insert(
                headername.into(),
                FullRequest {
                    method: self.method.clone(),
                    url: self.url.clone(),
                    headers: current_headers.clone(),
                    body: self.body.clone(),
                },
            );
        });
        results
    }
    fn inject_payloads(
        &mut self,
        payload: &str,
        remove_content: bool,
        injection_location: InjectionLocation,
    ) -> HashMap<String, FullRequest> {
        let mut injected_params = HashMap::new();
        match injection_location {
            InjectionLocation::Url => {
                let url_parser = HttpMessage {
                    url: Some(Url::parse(&self.url).unwrap()),
                };
                let iter_params = url_parser.change_urlquery(payload, remove_content);
                let mut results: HashMap<String, FullRequest> = HashMap::new();
                iter_params.iter().for_each(|(k, v)| {
                    results.insert(
                        k.to_string(),
                        FullRequest {
                            method: self.method.clone(),
                            url: v.to_string(),
                            headers: self.headers.clone(),
                            body: self.body.clone(),
                        },
                    );
                });
                injected_params.extend(iter_params);
                results
            }
            InjectionLocation::Headers => self.inject_headers(payload, remove_content),
            _ => HashMap::new(),
        }
    }
}

impl UserData for FullRequest {
    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method_mut(
            "set_url_param",
            |_, this, (payload, remove_param_content): (String, Option<bool>)| {
                let remove_content = remove_param_content.unwrap_or(false);
                let injected_params =
                    this.inject_payloads(&payload, remove_content, InjectionLocation::Url);
                Ok(injected_params)
            },
        );

        methods.add_method_mut(
            "set_path_param",
            |_, this, (payload, remove_param_content): (String, Option<bool>)| {
                let remove_content = remove_param_content.unwrap_or(false);
                let injected_params =
                    this.inject_payloads(&payload, remove_content, InjectionLocation::Path);
                Ok(injected_params)
            },
        );

        methods.add_method_mut(
            "set_body_param",
            |_, this, (payload, remove_param_content): (String, Option<bool>)| {
                let remove_content = remove_param_content.unwrap_or(false);
                let injected_params =
                    this.inject_payloads(&payload, remove_content, InjectionLocation::Body);
                Ok(injected_params)
            },
        );

        methods.add_method_mut(
            "set_headers_param",
            |_, this, (payload, remove_param_content): (String, Option<bool>)| {
                let remove_content = remove_param_content.unwrap_or(false);
                let injected_params =
                    this.inject_payloads(&payload, remove_content, InjectionLocation::Headers);
                Ok(injected_params)
            },
        );
    }
}
