/*
 * This file is part of Lotus Project, an Web Security Scanner written in Rust based on Lua Scripts
 * For details, please see https://github.com/rusty-sec/lotus/
 *
 * Copyright (c) 2022 - Khaled Nassar
 *
 * Please note that this file was originally released under the
 * GNU General Public License as published by the Free Software Foundation;
 * either version 2 of the License, or (at your option) any later version.
 *
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use mlua::UserData;
use reqwest::{
    header::{HeaderMap, HeaderName, HeaderValue},
    redirect, Client, Method, Proxy,
};
use std::{collections::HashMap, time::Duration};
use tealr::{mlu::FromToLua, TypeName};

#[derive(Clone)]
pub struct Sender {
    proxy: Option<String>,
    timeout: u64,
    redirects: u32,
}

/// RespType for lua userdata
#[derive(FromToLua, Clone, Debug, TypeName)]
pub enum RespType {
    NoErrors,
    Emtpy,
    Str(String),
    Int(i32),
    Headers(HashMap<String, String>),
    Error(String),
}

/// Adding OOP for http sender class
impl UserData for Sender {
    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method_mut("set_proxy", |_, this, the_proxy: mlua::Value| {
            match the_proxy {
                mlua::Value::String(new_proxy) => {
                    this.proxy = Some(new_proxy.to_str().unwrap().to_string());
                }
                _ => {
                    this.proxy = None;
                }
            };
            Ok(())
        });
        methods.add_method_mut("set_timeout", |_, this, timeout: u64| {
            this.timeout = timeout;
            Ok(())
        });

        methods.add_method_mut("set_redirects", |_, this, redirects: u32| {
            this.redirects = redirects;
            Ok(())
        });
        methods.add_async_method(
            "send",
            |_, this, (method, url, req_body, req_headers): (String, String, mlua::Value, mlua::Value)| async move {
                let body: String;
                let mut all_headers = HeaderMap::new();
                match req_headers {
                    mlua::Value::Table(current_headers) => {
                        current_headers.pairs::<String,String>().for_each(|currentheader| {
                            all_headers.insert(HeaderName::from_bytes(currentheader.clone().unwrap().0.as_bytes()).unwrap(), HeaderValue::from_bytes(currentheader.unwrap().1.as_bytes()).unwrap());
                        });
                    },
                    _ => {
                    }
                };
                body = match req_body {
                    mlua::Value::String(body) => {
                        body.to_str().unwrap().to_string()
                    },
                    _ => {
                        "".to_string()
                    }
                };

                let resp = this.send(&method, url, body, all_headers).await;
                Ok(resp)
            },
        );
    }
}

impl Sender {
    /// Build your own http request module with user option
    ///
    pub fn init(proxy: Option<String>, timeout: u64, redirects: u32) -> Sender {
        Sender {
            timeout,
            redirects,
            proxy,
        }
    }

    fn build_client(&self) -> Result<reqwest::Client, reqwest::Error> {
        match &self.proxy {
            Some(the_proxy) => {
                Client::builder()
                    .timeout(Duration::from_secs(self.timeout))
                    .redirect(redirect::Policy::limited(self.redirects as usize))
                    .proxy(Proxy::all(the_proxy).unwrap())
                    .no_trust_dns()
                    .user_agent(
                        "Mozilla/5.0 (X11; Manjaro; Linux x86_64; rv:100.0) Gecko/20100101 Firefox/100.0",
                    )
                    .danger_accept_invalid_certs(true)
                    .build()
            }
            None => {
                Client::builder()
                    .timeout(Duration::from_secs(self.timeout))
                    .redirect(redirect::Policy::limited(self.redirects as usize))
                    .no_proxy()
                    .no_trust_dns()
                    .user_agent(
                        "Mozilla/5.0 (X11; Manjaro; Linux x86_64; rv:100.0) Gecko/20100101 Firefox/100.0",
                    )
                    .danger_accept_invalid_certs(true)
                    .build()
            }
        }
    }
    /// Send http request to custom url with user options (proxy, headers, etc.)
    /// the response should be HashMap with RespType enum
    ///
    ///
    /// ```rust
    /// let mut headers = HashMap::new();
    /// headers.insert("API-KEY".into(),"123".into());
    /// let resp = http.send("PUT","http://example.com","post_id=1",headers)
    /// ```
    ///
    /// ***
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
    ) -> HashMap<String, RespType> {
        let mut resp_data: HashMap<String, RespType> = HashMap::new();
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
                let mut resp_headers: HashMap<String, String> = HashMap::new();
                resp.headers()
                    .iter()
                    .for_each(|(header_name, header_value)| {
                        resp_headers.insert(
                            header_name.to_string(),
                            header_value.to_str().unwrap().to_string(),
                        );
                    });
                resp_data.insert("url".to_string(), RespType::Str(url));
                resp_data.insert(
                    "status".to_string(),
                    RespType::Str(resp.status().to_string()),
                );
                resp_data.insert(
                    "body".to_string(),
                    RespType::Str(resp.text().await.unwrap()),
                );
                resp_data.insert("errors".to_string(), RespType::NoErrors);
                resp_data.insert("headers".to_string(), RespType::Headers(resp_headers));
                resp_data
            }
            Err(err) => {
                resp_data.insert("status".to_string(), RespType::Emtpy);
                resp_data.insert("body".to_string(), RespType::Emtpy);
                resp_data.insert("errors".to_string(), RespType::Error(err.to_string()));
                resp_data.insert("headers".to_string(), RespType::Headers(HashMap::new()));
                resp_data
            }
        }
    }
}
