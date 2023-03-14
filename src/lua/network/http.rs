// This file is part of Lotus Project, a web security scanner written in Rust based on Lua scripts.
// For details, please see https://github.com/rusty-sec/lotus/
//
// Copyright (c) 2022 - Khaled Nassar
//
// Please note that this file was originally released under the GNU General Public License as
// published by the Free Software Foundation; either version 2 of the License, or (at your option)
// any later version.
//
// Unless required by applicable law or agreed to in writing, software distributed under the
// License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND,
// either express or implied. See the License for the specific language governing permissions
// and limitations under the License.

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
    pub static ref VERBOSE_MODE: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));
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
                .redirect(redirect::Policy::custom(move |attempt| {
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
                .redirect(redirect::Policy::custom(move |attempt| {
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
            let req_limit = *REQUESTS_LIMIT.lock().unwrap();
            let mut req_sent = REQUESTS_SENT.lock().unwrap();
            if *req_sent >= req_limit {
                let sleep_time = *SLEEP_TIME.lock().unwrap();
                let bar = BAR.lock().unwrap();
                let msg = format!("The rate limit for requests has been reached. Sleeping for {} seconds...", sleep_time);
                bar.println(&msg);
                log::debug!("{}", msg);
                std::thread::sleep(Duration::from_secs(sleep_time));
                *req_sent = 1;
                bar.println("Continuing...");
                log::debug!("Resetting req_sent value to 1");
            } else {
                *req_sent += 1;
            }
        }

        let client = self.build_client().unwrap();
        let request = client
            .request(Method::from_bytes(method.as_bytes()).unwrap(), url.clone())
            .headers(headers)
            .body(body);

        let response = match request.send().await {
            Ok(resp) => {
                let verbose_mode = *VERBOSE_MODE.lock().unwrap();
                if verbose_mode {
                    let msg = format!("Sent HTTP request: {}", &url);
                    BAR.lock().unwrap().println(&msg);
                    log::debug!("{}", msg);
                }

                let mut resp_headers = HashMap::new();
                resp.headers().iter().for_each(|(name, value)| {
                    let value = String::from_utf8_lossy(value.as_bytes()).to_string();
                    resp_headers.insert(name.to_string(), value);
                });

                let url = resp.url().to_string();
                let status = resp.status().as_u16() as i32;
                let body = resp.bytes().await.map(|b| String::from_utf8_lossy(&b).to_string());
                match body {
                    Ok(body) => Ok(HttpResponse {
                        url,
                        status,
                        body,
                        headers: resp_headers,
                    }),
                    Err(_) => {
                        let err = mlua::Error::RuntimeError("Timeout Body".to_string());
                        log::error!("Timeout Body");
                        Err(err)
                    }
                }
            }
            Err(err) => {
                let error_code = match () {
                    _ if err.is_timeout() => "timeout_error",
                    _ if err.is_connect() => "connection_error",
                    _ if err.is_redirect() => "too_many_redirects",
                    _ if err.is_body() => "request_body_error",
                    _ if err.is_decode() => "decode_error",
                    _ => "external_error",
                };

                let err = mlua::Error::RuntimeError(error_code.to_string());
                Err(err)
            }
        };

        response
    }
}
