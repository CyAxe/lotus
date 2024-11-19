use slinger::{self, Request};

use slinger::http::{HeaderMap, Method};
use slinger::{Client, Response, RequestBuilder, Socket};
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

#[derive(Default)]
pub struct RequestOptions {
    pub headers: Option<HeaderMap>,
}

pub struct Requester {
    client: Arc<Client>,
}

impl Requester {
    pub fn new(options: RequestOptions) -> Result<Self, RequestError> {
        let mut client_builder = Client::builder();

        if let Some(headers) = options.headers {
            client_builder = client_builder.default_headers(headers);
        }

        let client = client_builder
            .build()
            .map_err(|e| RequestError::ClientError(format!("Failed to build HTTP client: {}", e)))?;

        Ok(Requester { client: Arc::new(client) })
    }

    pub async fn get<T: AsRef<str> + Send + Sync + 'static>(&self, url: T, options: Option<RequestOptions>) -> Result<Response, RequestError> {
        let mut request = self.client.get(url.as_ref());
        if let Some(opts) = options {
            if let Some(headers) = opts.headers {
                request = request.headers(headers);
            }
        }
        request.send().map_err(|e| RequestError::SendError(format!("GET request failed for URL {}: {}", url.as_ref(), e)))
    }

    pub async fn post<U: AsRef<str> + Send + Sync + 'static>(&self, url: U, options: Option<RequestOptions>) -> Result<Response, RequestError> {
        let mut request = self.client.post(url.as_ref());
        if let Some(opts) = options {
            if let Some(headers) = opts.headers {
                request = request.headers(headers);
            }
        }
        request.send().map_err(|e| RequestError::SendError(format!("POST request failed for URL {}: {}", url.as_ref(), e)))
    }

    pub fn send_raw_request<U: AsRef<str> + 'static, R: AsRef<str> + 'static>(&self, url: U, raw_request: R) -> Result<Response, RequestError> {
        let formatted_request = raw_request.as_ref().replace('\n', "\r\n");
        self.client
            .raw(url.as_ref(), formatted_request, true)
            .send()
            .map_err(|e| RequestError::SendError(format!("Raw request failed for URL {}: {}", url.as_ref(), e)))
    }

    pub fn request<U: AsRef<str>>(&self, method: Method, url: U, options: Option<RequestOptions>) -> RequestBuilder {
        let mut request = self.client.request(method, url.as_ref());
        if let Some(opts) = options {
            if let Some(headers) = opts.headers {
                request = request.headers(headers);
            }
        }
        request
    }

    pub fn execute_request(&self, socket: &mut Socket, request: &Request) -> Result<Response, RequestError> {
        self.client
            .execute_request(socket, request)
            .map_err(|e| RequestError::SendError(format!("Failed to execute request: {}", e)))
    }
}
