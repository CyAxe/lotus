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

use std::collections::HashMap;
use url::Url;
mod url_lua;

#[derive(Clone)]
pub struct HttpMessage {
    pub url: Url,
}

impl HttpMessage {
    pub fn change_urlquery(&self, payload: &str, remove_content: bool) -> HashMap<String, String> {
        let mut scan_params = HashMap::with_capacity(16);
        let mut result = HashMap::with_capacity(16);

        for (key, value) in self.url.query_pairs() {
            scan_params.insert(key.to_string(), value.to_string());
        }

        for (key, value) in scan_params.iter() {
            for pl in payload.split('\n') {
                let mut new_params = scan_params.clone();
                if remove_content {
                    new_params.insert(key.to_string(), pl.to_string());
                } else {
                    let mut new_value = String::with_capacity(value.len() + pl.len());
                    new_value.push_str(value);
                    new_value.push_str(pl);
                    new_params.insert(key.to_string(), new_value);
                }

                let mut new_url = self.url.clone();
                new_url.set_query(None);

                for (key, value) in new_params.iter() {
                    new_url.query_pairs_mut().append_pair(key, value);
                }

                result.insert(key.to_string(), new_url.as_str().to_string());
            }
        }

        result
    }

    pub fn set_urlvalue(&self, param: &str, payload: &str) -> String {
        let mut final_params = HashMap::with_capacity(16);

        for (key, value) in self.url.query_pairs() {
            if key == param {
                let mut new_value = String::with_capacity(value.len() + payload.len());
                new_value.push_str(value.as_ref().to_string().as_str());
                new_value.push_str(payload);
                final_params.insert(key.to_string(), new_value);
            } else {
                final_params.insert(key.to_string(), value.to_string());
            }
        }

        let mut new_url = self.url.clone();
        new_url.set_query(None);

        for (key, value) in final_params.iter() {
            new_url.query_pairs_mut().append_pair(key, value);
        }

        new_url.as_str().to_string()
    }

    pub fn urljoin(&self, path: &str) -> String {
        self.url.join(path).unwrap().as_str().to_string()
    }
}
