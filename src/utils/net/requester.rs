use slinger::{self, Request};

use slinger::http::{HeaderMap, Method};
use slinger::{Client, Response, Socket};
use std::sync::Arc;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RequestError {
    #[error("HTTP client error: {0}")]
    ClientError(String),
    #[error("Failed to send request: {0}")]
    SendError(String),
    #[error("Unexpected response error: {0}")]
    ResponseError(String),
}

pub struct Requester {
    client: Arc<Client>,
}

impl Requester {
    pub fn new() -> Result<Self, RequestError> {
        let client = Client::builder()
            .default_headers(HeaderMap::new())
            .cookie_store(true)
            .build()
            .map_err(|e| RequestError::ClientError(format!("Failed to build HTTP client: {}", e)))?;
        Ok(Requester { client: Arc::new(client) })
    }

    pub async fn get<T: AsRef<str> + Send + Sync + 'static>(&self, url: T) -> Result<Response, RequestError> {
        self.client
            .get(url.as_ref())
            .send()
            .map_err(|e| RequestError::SendError(format!("GET request failed for URL {}: {}", url.as_ref(), e)))
    }

    pub async fn post<U: AsRef<str> + Send + Sync + 'static, B: AsRef<str> + Send + Sync + 'static>(&self, url: U, body: B) -> Result<Response, RequestError> {
        self.client
            .post(url.as_ref())
            .body(body.as_ref().to_owned())
            .send()
            .map_err(|e| RequestError::SendError(format!("POST request failed for URL {}: {}", url.as_ref(), e)))
    }

    pub fn send_raw_request<U: AsRef<str> + 'static, R: AsRef<str> + 'static>(&self, url: U, raw_request: R) -> Result<Response, RequestError> {
        let formatted_request = raw_request.as_ref().replace('\n', "\r\n");
        self.client
            .raw(url.as_ref(), formatted_request, true)
            .send()
            .map_err(|e| RequestError::SendError(format!("Raw request failed for URL {}: {}", url.as_ref(), e)))
    }

    pub fn request_and_execute<U: AsRef<str>>(&self, method: Method, url: U, socket: &mut Socket) -> Result<Response, RequestError> {
        let request = self.client.request(method, url.as_ref()).build().map_err(|e| RequestError::ClientError(format!("Failed to build request for URL {}: {}", url.as_ref(), e)))?;
        self.client
            .execute_request(socket, &request)
            .map_err(|e| RequestError::SendError(format!("Failed to execute request: {}", e)))
    }
}
