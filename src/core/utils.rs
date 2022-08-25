use std::collections::HashMap;

pub struct Sender {}

impl Sender {
    pub(crate) fn init() -> Sender {
        Sender {}
    }
    pub(crate) fn send(&self, url: String) -> Result<HashMap<String, String>, reqwest::Error> {
        let mut resp_data: HashMap<String, String> = HashMap::new();
        match reqwest::blocking::get(url) {
            Ok(resp) => {
                resp_data.insert("url".to_string(), resp.url().to_string());
                resp_data.insert("status".to_string(), resp.status().to_string());
                resp_data.insert("body".to_string(),resp.text().unwrap());
                Ok(resp_data)
            }
            Err(err) => Err(err),
        }
    }
}
