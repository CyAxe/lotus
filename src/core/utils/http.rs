use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tealr::{rlu::FromToLua, TypeName};

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
}

impl Sender {
    pub fn init() -> Sender {
        Sender {
            many_sent: Arc::new(Mutex::new(0)),
        }
    }
    pub fn send(&mut self, url: String) -> HashMap<String, RespType> {
        let mut resp_data: HashMap<String, RespType> = HashMap::new();
        match reqwest::blocking::get(url) {
            Ok(resp) => {
                *self.many_sent.lock().unwrap() += 1;
                resp_data.insert("url".to_string(), RespType::Str(resp.url().to_string()));
                resp_data.insert(
                    "status".to_string(),
                    RespType::Str(resp.status().to_string()),
                );
                resp_data.insert(
                    "body".to_string(),
                    RespType::Str(resp.text().unwrap_or("".to_string())),
                );
                resp_data.insert("errors".to_string(), RespType::NoErrors);
                resp_data
            }
            Err(err) => {
                resp_data.insert("url".to_string(), RespType::Emtpy);
                resp_data.insert("status".to_string(), RespType::Emtpy);
                resp_data.insert("body".to_string(), RespType::Emtpy);
                resp_data.insert("errors".to_string(), RespType::Error(err.to_string()));
                resp_data
            }
        }
    }
}
