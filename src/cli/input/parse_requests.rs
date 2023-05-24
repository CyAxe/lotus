use mlua::UserData;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use reqwest::Url;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tealr::TypeName;

use crate::lua::network::http::HttpResponse;
use crate::lua::network::http::Sender;
use crate::lua::parsing::url::HttpMessage;

#[derive(TypeName, Clone, Deserialize, Serialize)]
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
    fn inject_headers(&self, payload: &str, remove_content: bool) -> HashMap<String, FullRequest> {
        let mut results = HashMap::new();
        let headers = &self.headers;

        let cloned_headers = headers.clone();
        for (headername, headervalue) in headers.iter() {
            let mut current_headers = cloned_headers.clone();
            let mut current_results = HashMap::new();
            let new_headervalue = if remove_content {
                format!("{}{}", headervalue, payload)
            } else {
                format!("{}", payload)
            };
            current_headers.insert(headername.clone(), new_headervalue.clone());
            current_results.insert(headername.clone(), new_headervalue);
            let new_request = FullRequest {
                method: self.method.clone(),
                url: self.url.clone(),
                headers: current_headers,
                body: self.body.clone(),
            };
            results.insert(headername.clone(), new_request);
        }
        results
    }

    async fn send(
        &self,
        request: FullRequest,
        request_opts: Sender,
    ) -> Result<HttpResponse, mlua::Error> {
        let method = request.method;
        let url = request.url;
        let current_headers = request.headers;
        let mut headers: HeaderMap<HeaderValue>;
        let mut request_opts = request_opts.clone();
        let body = request.body;
        let sender = request_opts.clone();
        if sender.merge_headers {
            headers = request_opts.headers;
            current_headers
                .iter()
                .for_each(|(header_name, header_value)| {
                    headers.insert(
                        HeaderName::from_bytes(header_name.as_bytes()).unwrap(),
                        HeaderValue::from_str(header_value).unwrap(),
                    );
                });
        } else {
            let mut header_map = HeaderMap::new();
            current_headers
                .iter()
                .for_each(|(header_name, header_value)| {
                    header_map.insert(
                        HeaderName::from_bytes(header_name.as_bytes()).unwrap(),
                        HeaderValue::from_str(header_value).unwrap(),
                    );
                });
            headers = header_map;
        }
        request_opts.headers = headers;
        Ok(request_opts
            .send(&method, url, Some(body), None, request_opts.clone())
            .await?)
    }

    fn inject_body(&self, payload: &str, remove_content: bool) -> HashMap<String, FullRequest> {
        let mut results = HashMap::new();
        let req_body = &self.body;
        let parsed_params = url::form_urlencoded::parse(req_body.as_bytes())
            .into_owned()
            .collect::<HashMap<String, String>>();
        parsed_params.iter().for_each(|(param, _)| {
            let mut current_req = self.clone();
            let mut current_parsed_param = parsed_params.clone();
            if let Some(value) = current_parsed_param.get_mut(param) {
                *value = if remove_content {
                    payload.to_string()
                } else {
                    value.to_string() + payload
                }
            }
            current_req.body = serde_urlencoded::to_string(current_parsed_param).unwrap();
            results.insert(param.clone(), current_req.clone());
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
                let (method, headers, body) =
                    (self.method.clone(), self.headers.clone(), self.body.clone());
                let results: Vec<(String, FullRequest)> = iter_params
                    .iter()
                    .map(|(k, v)| {
                        (
                            k.to_string(),
                            FullRequest {
                                method: method.clone(),
                                url: v.to_string(),
                                headers: headers.clone(),
                                body: body.clone(),
                            },
                        )
                    })
                    .collect();
                injected_params.extend(iter_params);
                results.into_iter().collect()
            }
            InjectionLocation::Headers => self.inject_headers(payload, remove_content),
            InjectionLocation::Body => self.inject_body(payload, remove_content),
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
        methods.add_async_method(
            "send",
            |_, this, (req, sender): (FullRequest, Sender)| async move {
                Ok(this.send(req, sender).await?)
            },
        );
    }
}
