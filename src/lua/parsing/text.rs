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
use regex::RegexBuilder;

#[derive(TypeName, Clone, Debug)]
pub struct ResponseMatcher {
    pub multi_line: bool,
    pub case_insensitive: bool,
    pub ignore_whitespace: bool,
    pub unicode: bool,
    pub octal: bool,
    pub dot_matches_new_line: bool
}

impl ResponseMatcher {
    pub fn match_and_body(&self, body: &str, text: Vec<String>, is_regex: Option<bool>) -> bool {
        let mut counter = 0;
        for x in text.iter() {
            if is_regex.unwrap_or(false) {
                if let Ok(re_pattern) = RegexBuilder::new(x)
                    .multi_line(self.multi_line)
                    .case_insensitive(self.case_insensitive)
                    .unicode(self.unicode)
                    .octal(self.octal)
                    .dot_matches_new_line(self.dot_matches_new_line)
                    .build() {
                    if re_pattern.is_match(body) {
                        counter += 1;
                    }
                }
            } else if body.contains(x) {
                counter += 1;
            }
        }
        counter == text.len()
    }

    pub fn match_once_body(&self, body: String, text: Vec<String>, is_regex: Option<bool>) -> Vec<String> {
        let mut matched_data = Vec::new();
        for pattern in text {
            if is_regex.unwrap_or(false) {
                if let Ok(re) = RegexBuilder::new(&pattern)
                    .multi_line(self.multi_line)
                    .case_insensitive(self.case_insensitive)
                    .unicode(self.unicode)
                    .octal(self.octal)
                    .dot_matches_new_line(self.dot_matches_new_line)
                    .build() {
                    if re.is_match(&body) {
                        matched_data.push(pattern);
                    }
                }
            } else if body.contains(&pattern) {
                matched_data.push(pattern);
            }
        }
        matched_data
    }
}

impl UserData for ResponseMatcher {
    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method(
            "match_body",
            |_, this, (response, text_list, is_regex): (String, Vec<String>, Option<bool>)| {
                Ok(this.match_and_body(&response, text_list,is_regex))
            },
        );
        methods.add_method(
            "match_body_once",
            |_, this, (response, text_list, is_regex): (String, Vec<String>, Option<bool>)| {
                let is_match = this.match_once_body(response, text_list,is_regex);
                Ok(is_match)
            },
        );
        methods.add_method_mut("options", |_, this, opts: mlua::Table| {
            let response_matcher = ResponseMatcher {
                multi_line: opts.get::<_, bool>("multi_line").unwrap_or(this.multi_line),
                case_insensitive: opts.get::<_, bool>("case_insensitive").unwrap_or(this.case_insensitive),
                ignore_whitespace: opts.get::<_, bool>("ignore_whitespace").unwrap_or(this.ignore_whitespace),
                unicode: opts.get::<_, bool>("unicode").unwrap_or(this.unicode),
                octal: opts.get::<_, bool>("octal").unwrap_or(this.octal),
                dot_matches_new_line: opts.get::<_, bool>("dot_matches_new_line").unwrap_or(this.dot_matches_new_line),
            };

            *this = response_matcher;

            Ok(())
        });
    }
}
