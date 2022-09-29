use log::debug;
pub mod bar;
pub mod files;
pub mod html;
pub mod http;
pub mod url;

pub fn is_match(pattern: String, resp: String) -> bool {
    let re = fancy_regex::Regex::new(&pattern);
    if re.is_ok() {
        re.unwrap().is_match(&resp).unwrap_or(false)
    } else {
        debug!("MATCHING ERROR  {:?} | {:?}", pattern, re);
        false
    }
}
