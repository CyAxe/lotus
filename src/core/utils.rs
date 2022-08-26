use std::collections::HashMap;

pub struct Sender {}

impl Sender {
    pub fn init() -> Sender {
        Sender {}
    }
    pub fn send(&self, url: String) -> Result<HashMap<String, String>, reqwest::Error> {
        let mut resp_data: HashMap<String, String> = HashMap::new();
        match reqwest::blocking::get(url) {
            Ok(resp) => {
                resp_data.insert("url".to_string(), resp.url().to_string());
                resp_data.insert("status".to_string(), resp.status().to_string());
                resp_data.insert("body".to_string(), resp.text().unwrap());
                Ok(resp_data)
            }
            Err(err) => Err(err),
        }
    }
}

pub fn regex(pattern: (String,String)) -> bool {
    let re = fancy_regex::Regex::new(&pattern.0).unwrap();
    re.is_match(&pattern.1).unwrap_or(false)
}
