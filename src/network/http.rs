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

use reqwest::{header::HeaderMap, redirect, Client, Method, Proxy};
use std::{collections::HashMap, time::Duration};
mod http_lua_api;
pub use http_lua_api::Sender;
use tealr::{mlu::FromToLua, TypeName};

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
        match &self.proxy {
            Some(the_proxy) => Client::builder()
                .timeout(Duration::from_secs(self.timeout))
                .redirect(redirect::Policy::limited(self.redirects as usize))
                .default_headers(self.headers.clone())
                .proxy(Proxy::all(the_proxy).unwrap())
                .no_trust_dns()
                .danger_accept_invalid_certs(true)
                .build(),
            None => Client::builder()
                .timeout(Duration::from_secs(self.timeout))
                .redirect(redirect::Policy::limited(self.redirects as usize))
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
