/*
 * this file is part of lotus project, an web security scanner written in rust based on lua scripts
 * for details, please see https://github.com/rusty-sec/lotus/
 *
 * copyright (c) 2022 - khaled nassar
 *
 * please note that this file was originally released under the
 * gnu general public license as published by the free software foundation;
 * either version 2 of the license, or (at your option) any later version.
 *
 *
 * unless required by applicable law or agreed to in writing, software
 * distributed under the license is distributed on an "as is" basis,
 * without warranties or conditions of any kind, either express or implied.
 * see the license for the specific language governing permissions and
 * limitations under the license.
 */

use crate::BAR;
use reqwest::{header::HeaderMap, redirect, Client, Method, Proxy};
use std::collections::HashMap;
mod http_lua_api;
pub use http_lua_api::Sender;
use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tealr::{mlu::FromToLua, TypeName};

lazy_static! {
    pub static ref REQUESTS_LIMIT: Arc<Mutex<i32>> = Arc::new(Mutex::new(5));
    pub static ref REQUESTS_SENT: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));
    pub static ref SLEEP_TIME: Arc<Mutex<u64>> = Arc::new(Mutex::new(5));
}

#[derive(Debug, FromToLua, TypeName)]
pub struct HttpResponse {
    pub url: String,
    pub status: i32,
    pub body: String,
    pub headers: HashMap<String, String>,
}

impl Sender {
    /// Build your own http request module with user option
    ///
    pub fn init(headers: HeaderMap, proxy: Option<String>, timeout: u64, redirects: u32) -> Sender {
        Sender {
            headers,
            timeout,
            redirects,
            proxy,
        }
    }

    fn build_client(&self) -> Result<reqwest::Client, reqwest::Error> {
        let redirects = self.redirects;
        match &self.proxy {
            Some(the_proxy) => Client::builder()
                .timeout(Duration::from_secs(self.timeout))
                .redirect(redirect::Policy::custom(move |attempt|{
                    if attempt.previous().len() != redirects as usize {
                        attempt.follow()
                    } else {
                        attempt.stop()
                    }
                }))
                .default_headers(self.headers.clone())
                .proxy(Proxy::all(the_proxy).unwrap())
                .no_trust_dns()
                .danger_accept_invalid_certs(true)
                .build(),
            None => Client::builder()
                .timeout(Duration::from_secs(self.timeout))
                .redirect(redirect::Policy::custom(move |attempt|{
                    if attempt.previous().len() == redirects as usize {
                        attempt.stop()
                    } else {
                        attempt.follow()
                    }
                }))
                .no_proxy()
                .no_trust_dns()
                .default_headers(self.headers.clone())
                .danger_accept_invalid_certs(true)
                .build(),
        }
    }
    /// Send http request to custom url with user options (proxy, headers, etc.)
    /// the response should be HashMap with RespType enum
    ///
    /// for Lua API
    /// ```lua
    /// local resp = http:send("GET","http://google.com")
    /// print(resp.body:GetStrOrNil())
    ///
    /// -- set proxy/timeout    
    /// http:set_proxy("http://proxysite.com:8080")
    /// http:set_timeout(15)
    /// http:set_redirects(2) // set custom redirects limit
    /// ```
    pub async fn send(
        &self,
        method: &str,
        url: String,
        body: String,
        headers: HeaderMap,
    ) -> Result<HttpResponse, mlua::Error> {
        {
            let req_limit = REQUESTS_LIMIT.lock().unwrap();
            let mut req_sent = REQUESTS_SENT.lock().unwrap();
            if *req_sent >= *req_limit {
                let sleep_time = SLEEP_TIME.lock().unwrap();
                let bar = BAR.lock().unwrap();
                bar.println(format!(
                    "The rate limit for requests has been raised, please wait {} seconds ",
                    *sleep_time
                ));
                log::debug!(
                    "{}",
                    format!(
                        "The rate limit for requests has been raised, please wait {} seconds ",
                        *sleep_time
                    )
                );
                std::thread::sleep(Duration::from_secs(*sleep_time));
                *req_sent = 1;
                bar.println("Continue ...");
                log::debug!("changing req_sent value to 1");
            }
        };
        match self
            .build_client()
            .unwrap()
            .request(Method::from_bytes(method.as_bytes()).unwrap(), url.clone())
            .headers(headers)
            .body(body)
            .send()
            .await
        {
            Ok(resp) => {
                // Locking Scope
                {
                    let mut req_sent = REQUESTS_SENT.lock().unwrap();
                    *req_sent += 1;
                };
                let mut resp_headers: HashMap<String, String> = HashMap::new();
                resp.headers()
                    .iter()
                    .for_each(|(header_name, header_value)| {
                        resp_headers.insert(
                            header_name.to_string(),
                            String::from_utf8_lossy(header_value.as_bytes()).to_string(),
                        );
                    });
                let url = resp.url().to_string();
                let status = resp.status().as_u16() as i32;
                let body = resp.bytes().await.unwrap();
                let body = String::from_utf8_lossy(&body).to_string();
                let resp_data_struct = HttpResponse {
                    url,
                    status,
                    body,
                    headers: resp_headers,
                };
                Ok(resp_data_struct)
            }
            Err(err) => {
                let error_code = {
                    if err.is_timeout() {
                        "timeout_error"
                    } else if err.is_connect(){
                        "connection_error"
                    } 
                    else if err.is_redirect() {
                        "too_many_redirects"
                    }
                    else if err.is_body() {
                        "request_body_error" 
                    } else if err.is_decode() {
                        "decode_error"
                    } else {
                        "external_error"
                    }
                };
                let err = mlua::Error::RuntimeError(error_code.to_string());
                Err(err)
            },
        }
    }
}
