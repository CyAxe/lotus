use content_inspector::{inspect, ContentType};
use reqwest::header::HeaderMap;
use reqwest::Url;
use std::collections::HashMap;
use crate::core::utils::parsing::url::HttpMessage;

pub enum InjectionPoint {
    urlValue,
    Headers,
    Body,
}

pub enum BodyType {
    Form,
    Json,
    Binary,
}


pub struct ReqHandler {
    pub method: String,
    pub url: String,
    pub body: String,
    pub headers: HeaderMap,
    pub inject_point: Vec<InjectionPoint>,
    pub body_type: BodyType,
    pub current_params: Option<Vec<String>>,
    pub current_param_id: Option<i32>,
    pub current_point: Option<InjectionPoint>
}

impl ReqHandler {
    pub fn init(
        method: String,
        url: String,
        headers: HeaderMap,
        body: String,
        inject_point: Vec<InjectionPoint>,
    ) -> ReqHandler {
        let body_type = {
            if inspect(body.as_bytes()) == ContentType::BINARY {
                BodyType::Binary
            } else {
                match serde_json::from_str(&body) {
                    Ok(()) => BodyType::Json,
                    Err(_) => BodyType::Form,
                }
            }
        };

        ReqHandler {
            method,
            url,
            headers,
            body,
            inject_point,
            body_type,
            current_params: None,
            current_param_id: None,
            current_point: None
        }
    }

    fn set_body(&mut self) {
        match self.body_type {
            BodyType::Form => {}
            BodyType::Json => {}
            BodyType::Binary => {}
        };
    }
    fn set_urlvalue(&mut self) {}

    fn change_payload(&mut self) {
    }
    fn change_point(&mut self, inject_point: InjectionPoint) {
        if !(self.current_params.is_some()){
            let mut new_params = Vec::new();
            Url::parse(&self.url).expect("f").query_pairs()
                .collect::<HashMap<_, _>>().iter().for_each(|(key,value)| {
                    new_params.push(key.to_string());
                });
            self.current_params = Some(new_params);
        }
    } 
    pub fn set_payload(&mut self, _payload: String) {
    }
}
