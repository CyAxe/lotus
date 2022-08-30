use reqwest::Url;
use scraper::Html;
use scraper::Selector;
use std::collections::HashMap;
use tealr::{rlu::FromToLua, TypeName};
pub mod files;

#[derive(Clone)]
pub struct Sender {}

#[derive(FromToLua, Clone, TypeName)]
struct RespStatus {
    test_field: String,
}
impl From<String> for RespStatus {
    fn from(t: String) -> Self {
        println!("started {}", &t);
        RespStatus { test_field: t }
    }
}

impl From<RespStatus> for String {
    fn from(t: RespStatus) -> Self {
        t.test_field
    }
}

#[derive(FromToLua, Clone, Debug, TypeName)]
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
                resp_data.insert(
                    "status".to_string(),
                    RespType::Str(resp.status().to_string()),
                );
                resp_data.insert("body".to_string(), RespType::Str(resp.text().unwrap()));
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

pub fn is_match(pattern: String, resp: String) -> bool {
    let re = fancy_regex::Regex::new(&pattern).unwrap();
    re.is_match(&resp).unwrap_or(false)
}

#[derive(FromToLua, Clone, Debug, TypeName)]
#[tealr(creator_name = LocationMaker)]
#[tealr(extend_methods = method_extension)]
pub enum Location {
    AttrValue(String),
    AttrName(String),
    TagName(String),
    Text(String),
    Comment(String),
}

pub fn html_search(html: &str, pattern: &str) -> String {
    let mut found = String::new();
    let document = Html::parse_document(html);
    let select = Selector::parse(pattern).unwrap();
    for node in document.select(&select) {
        found.push_str(&node.html());
    }
    found
}

pub fn css_selector(html: &str) -> String {
    let mut found = String::new();
    let document = Html::parse_document(html);
    document.tree.values().for_each(|node| {
        if node.as_element().is_some() {
            let element = node.as_element().unwrap();
            let mut search = format!("{}", element.name());
            element.attrs.iter().for_each(|attr| {
                if attr.1.to_string().len() > 0 {
                    search.push_str(&format!(
                        r#"[{}="{}"]"#,
                        attr.0.local.to_string(),
                        attr.1.to_string().replace("'", "\\'").replace("\"", "\\\"")
                    ));
                } else {
                    search.push_str(&format!(r#"[{}]"#, attr.0.local.to_string()));
                }
            });
            if search.contains("[") {
                found.push_str(&search);
            }
        }
    });
    found
}

pub fn html_parse(html: &str, payload: &str) -> Vec<Location> {
    let mut found: Vec<Location> = Vec::new();
    if payload.len() == 0 {
        return found;
    }
    let document = Html::parse_document(html);
    document.tree.values().for_each(|node| {
        if node.is_text() {
            let text = node.as_text().unwrap();
            if text.contains(payload) {
                found.push(Location::Text(text.to_string()));
            }
        } else if node.is_element() {
            let element = node.as_element().unwrap();
            if element.name().contains(payload) {
                found.push(Location::TagName(element.name().to_string()));
            }
            element.attrs().for_each(|attr| {
                if attr.1.contains(payload) {
                    found.push(Location::AttrValue(attr.1.to_string()));
                }
                if attr.0.contains(payload) {
                    found.push(Location::AttrName(attr.0.to_string()));
                }
            });
        } else if node.is_comment() {
            let comment = node.as_comment().unwrap();
            if comment.contains(payload) {
                found.push(Location::Comment(comment.to_string()));
            }
        }
    });
    found
}

pub fn change_urlquery(url: String, payload: String) -> HashMap<String, String> {
    let url = Url::parse(&url).unwrap();
    let params: HashMap<_, _> = url.query_pairs().collect::<HashMap<_, _>>();
    let mut scan_params = HashMap::new();
    let mut result: HashMap<String, String> = HashMap::new();
    let mut param_list = Vec::new();
    params.iter().for_each(|(key, value)| {
        scan_params.insert(key.to_string(), value.to_string());
        param_list.push(key.to_string());
    });
    drop(params);

    scan_params.iter().for_each(|(key, value)| {
        payload.split("\n").into_iter().for_each(|payload| {
            let mut new_params = scan_params.clone();
            new_params.insert(key.to_string(), value.as_str().to_owned() + payload);
            let mut new_url = url.clone();
            new_url.query_pairs_mut().clear();

            new_url.query_pairs_mut().extend_pairs(&new_params);

            result.insert(key.to_string(), new_url.as_str().to_string());
        });
    });
    result
}

pub fn set_urlvalue(url: &str, param: &str, payload: &str) -> String {
    let mut url = Url::parse(url).unwrap();
    let mut final_params = HashMap::new();
    url.query_pairs()
        .into_iter()
        .collect::<HashMap<_, _>>()
        .iter()
        .for_each(|(k, v)| {
            if k == param {
                final_params.insert(k.to_string(), format!("{}{}", v.to_string(), payload));
            } else {
                final_params.insert(k.to_string(), v.to_string());
            }
        });
    url.query_pairs_mut().clear();
    url.query_pairs_mut().extend_pairs(final_params);
    url.as_str().to_string()
}
