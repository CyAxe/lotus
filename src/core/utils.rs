use std::collections::HashMap;
use tealr::{TypeName,rlu::FromToLua};

pub struct Sender {}

#[derive(FromToLua,Clone,TypeName)]
struct RespStatus {
    test_field: String
 }
 impl From<String> for RespStatus {
    fn from(t : String) -> Self {
        println!("started {}",&t);
        RespStatus{test_field:t}
    }
 }

 impl From<RespStatus> for String {
    fn from(t: RespStatus) -> Self {
        t.test_field
    }
 }

#[derive(FromToLua,Clone,Debug,TypeName)]
#[tealr(creator_name = RespMaker)]
#[tealr(extend_methods = method_extension)]
pub enum RespType {
    NoErrors,
    Emtpy,
    Str(String),
    Int(i32),
    Error(String),
}


impl Sender {
    pub fn init() -> Sender {
        Sender {}
    }
    pub fn send(&self, url: String) -> HashMap<String, RespType> {
        let mut resp_data: HashMap<String, RespType> = HashMap::new();
        match reqwest::blocking::get(url) {
            Ok(resp) => {
                resp_data.insert("url".to_string(), RespType::Str(resp.url().to_string()));
                resp_data.insert("status".to_string(), RespType::Str(resp.status().to_string()));
                resp_data.insert("body".to_string(), RespType::Str(resp.text().unwrap()));
                resp_data.insert("errors".to_string(),RespType::NoErrors);
                resp_data
            }
            Err(err) => {
                resp_data.insert("url".to_string(), RespType::Emtpy);
                resp_data.insert("status".to_string(), RespType::Emtpy);
                resp_data.insert("body".to_string(), RespType::Emtpy);
                resp_data.insert("errors".to_string(), RespType::Error(err.to_string()));
                resp_data
            },
        }
    }
}

pub fn regex(pattern: (String,String)) -> bool {
    let re = fancy_regex::Regex::new(&pattern.0).unwrap();
    re.is_match(&pattern.1).unwrap_or(false)
}

