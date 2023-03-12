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

use mlua::UserData;
use tealr::TypeName;

#[derive(TypeName, Debug)]
pub struct ResponseMatcher {}

impl ResponseMatcher {
    pub fn match_and_body(&self, body: String, text: Vec<String>) -> bool {
        let mut counter = 0;
        text.iter().for_each(|x| {
            if body.contains(x) {
                counter += 1;
            }
        });
        if counter == text.len() {
            true
        } else {
            false
        }
    }
    pub fn match_once_body(&self, body: String, text: Vec<String>) -> String {
        let mut matched_data = "".into();
        text.iter().for_each(|x| {
            if body.contains(x) {
                matched_data = x.to_string();
            }
        });
        matched_data
    }
}

impl UserData for ResponseMatcher {
    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method(
            "match_body",
            |_, this, (response, text_list): (String, Vec<String>)| {
                Ok(this.match_and_body(response, text_list))
            },
        );
        methods.add_method(
            "match_body_once",
            |_, this, (response, text_list): (String, Vec<String>)| {
                let is_match = this.match_once_body(response, text_list);
                Ok(is_match)
            },
        )
    }
}
