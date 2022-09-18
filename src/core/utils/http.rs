use isahc::prelude::*;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tealr::{mlu::FromToLua, TypeName};

#[derive(Clone)]
pub struct Sender {
    pub many_sent: Arc<Mutex<i8>>,
}

#[derive(FromToLua, Clone, Debug, TypeName)]
pub enum RespType {
    NoErrors,
    Emtpy,
    Str(String),
    Int(i32),
    Error(String),
    Headers(HashMap<String, String>),
}

impl Sender {
    pub fn init() -> Sender {
        Sender {
            many_sent: Arc::new(Mutex::new(0)),
        }
    }
    pub async fn send(&mut self, url: String) -> HashMap<String, RespType> {
        let mut resp_data: HashMap<String, RespType> = HashMap::new();
        match isahc::get_async(url).await {
            Ok(mut resp) => {
                *self.many_sent.lock().unwrap() += 1;
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
                let mut response_headers: HashMap<String, String> = HashMap::new();
                resp.headers()
                    .iter()
                    .for_each(|(header_name, header_value)| {
                        response_headers.insert(
                            header_name.to_string(),
                            header_value.to_str().unwrap().to_string(),
                        );
                    });

                resp_data.insert("headers".to_string(), RespType::Headers(response_headers));
                resp_data.insert("errors".to_string(), RespType::NoErrors);
                resp_data
            }
            Err(err) => {
                resp_data.insert("status".to_string(), RespType::Emtpy);
                resp_data.insert("body".to_string(), RespType::Emtpy);
                resp_data.insert("headers".to_string(), RespType::Emtpy);
                resp_data.insert("errors".to_string(), RespType::Error(err.to_string()));
                resp_data
            }
        }
    }
}
