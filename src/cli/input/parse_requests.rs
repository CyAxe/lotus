use mlua::UserData;
use reqwest::header::{HeaderName, HeaderValue};
use reqwest::Url;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
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
        mut request_opts: Sender,
    ) -> Result<HttpResponse, mlua::Error> {
        let FullRequest {
            method,
            url,
            headers: current_headers,
            body,
        } = request;

        let headers = if request_opts.merge_headers {
            request_opts.headers.extend(current_headers.iter().map(|(name, value)| {
                (
                    HeaderName::from_bytes(name.as_bytes()).unwrap(),
                    HeaderValue::from_str(value).unwrap(),
                )
            }));
            request_opts.headers
        } else {
            current_headers
                .iter()
                .map(|(name, value)| {
                    (
                        HeaderName::from_bytes(name.as_bytes()).unwrap(),
                        HeaderValue::from_str(value).unwrap(),
                    )
                })
                .collect()
        };

        request_opts.headers = headers;

        request_opts.send(&method, url, Some(body), None, request_opts.clone()).await
    }

    fn inject_body_json(&self, payload: &str, remove_content: bool) -> HashMap<String, FullRequest> {
        let mut results = HashMap::new();
        let req_body = &self.body;

        // Parse the JSON body
        let parsed_params: Value = serde_json::from_str(req_body).unwrap_or_else(|_| json!({}));

        // Iterate over each key-value pair in the parsed JSON
        if let Some(params) = parsed_params.as_object() {
            for (param, value) in params.iter() {
                let mut current_req = self.clone();

                // Modify the value based on the 'remove_content' flag
                let mut value = value.clone();
                self.modify_value(&mut value, payload, remove_content);
                let mut current_parsed_params = parsed_params.clone();
                current_parsed_params[param] = value;

                // Convert the modified JSON back to a string
                let modified_body = serde_json::to_string(&current_parsed_params).unwrap();

                current_req.body = modified_body;
                results.insert(param.clone(), current_req.clone());
            }
        }

        results
    }


    fn modify_value(&self,value: &mut Value, payload: &str, remove_content: bool) {
        match value {
            Value::String(s) => {
                *s = if remove_content {
                    payload.to_string()
                } else {
                    s.to_string() + payload
                };
            }
            Value::Array(arr) => {
                for item in arr.iter_mut() {
                    self.modify_value(item, payload, remove_content);
                }
            }
            Value::Object(obj) => {
                for (_, item) in obj.iter_mut() {
                    self.modify_value(item, payload, remove_content);
                }
            }
            _ => {}
        }
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
        methods.add_method_mut("set_json_param", |_,this, (payload, remove_param_content): (String, Option<bool>)| {
            let remove_content = remove_param_content.unwrap_or(false);
            let inject_params = this.inject_body_json(&payload, remove_content);
            Ok(inject_params)
        });
        methods.add_async_method(
            "send",
            |_, this, (req, sender): (FullRequest, Sender)| async move {
                Ok(this.send(req, sender).await?)
            },
        );
    }
}
