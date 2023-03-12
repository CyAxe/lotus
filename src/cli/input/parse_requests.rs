use serde::{Deserialize, Serialize};
use std::collections::HashMap;


#[derive(Deserialize, Serialize)]
pub struct Request {
    pub method: String,
    pub url: String,
    pub headers: HashMap<String, String>,
    pub body: String
}


#[derive(Serialize, Deserialize)]
#[repr(transparent)]
pub struct ParsedRequests {
    pub requests: Vec<Request>
}
