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

use crate::cli::errors::CliErrors;
use mlua::ExternalError;
use mlua::UserData;
use regex::RegexBuilder;
use tealr::TypeName;

#[derive(TypeName, Clone, Debug)]
pub struct ResponseMatcher {
    pub multi_line: bool,
    pub case_insensitive: bool,
    pub ignore_whitespace: bool,
    pub unicode: bool,
    pub octal: bool,
    pub dot_matches_new_line: bool,
}

impl ResponseMatcher {
    /// Checks if the given regex pattern matches the response.
    pub fn is_match(&self, pattern: String, resp: String) -> Result<bool, CliErrors> {
        match RegexBuilder::new(&pattern)
            .multi_line(self.multi_line)
            .case_insensitive(self.case_insensitive)
            .unicode(self.unicode)
            .octal(self.octal)
            .dot_matches_new_line(self.dot_matches_new_line)
            .build()
        {
            Ok(re) => Ok(re.is_match(&resp)),
            Err(err) => {
                log::error!("Regex error: {}", err);
                Err(CliErrors::RegexPatternError)
            }
        }
    }

    /// Extracts all matches of the given regex pattern from the response.
    pub fn extract_data(
        &self,
        regex_pattern: &str,
        response: &str,
    ) -> Result<Vec<String>, CliErrors> {
        match RegexBuilder::new(regex_pattern)
            .multi_line(self.multi_line)
            .case_insensitive(self.case_insensitive)
            .unicode(self.unicode)
            .octal(self.octal)
            .dot_matches_new_line(self.dot_matches_new_line)
            .build()
        {
            Ok(re) => Ok(re.find_iter(response).map(|m| m.as_str().to_owned()).collect()),
            Err(err) => {
                log::error!("Regex error: {}", err);
                Err(CliErrors::RegexPatternError)
            }
        }
    }

    /// Replaces the first two matches of the regex pattern in the response with the replacement string.
    pub fn replace_txt(
        &self,
        regex_pattern: &str,
        replacement: &str,
        response: &str,
    ) -> Result<String, CliErrors> {
        match RegexBuilder::new(regex_pattern)
            .multi_line(self.multi_line)
            .case_insensitive(self.case_insensitive)
            .unicode(self.unicode)
            .octal(self.octal)
            .dot_matches_new_line(self.dot_matches_new_line)
            .build()
        {
            Ok(re) => Ok(re.replacen(response, 2, replacement).to_string()),
            Err(err) => {
                log::error!("Regex error: {}", err);
                Err(CliErrors::RegexPatternError)
            }
        }
    }

    /// Verifies if all provided patterns exist in the response body.
    pub fn match_and_body(
        &self,
        body: &str,
        text: Vec<String>,
        is_regex: Option<bool>,
    ) -> Result<bool, CliErrors> {
        let mut counter = 0;
        for search_pattern in text.iter() {
            match is_regex.unwrap_or(false) {
                true => {
                    match RegexBuilder::new(search_pattern)
                        .multi_line(self.multi_line)
                        .case_insensitive(self.case_insensitive)
                        .unicode(self.unicode)
                        .octal(self.octal)
                        .dot_matches_new_line(self.dot_matches_new_line)
                        .build()
                    {
                        Ok(re) => {
                            if re.is_match(body) {
                                counter += 1;
                            }
                        }
                        Err(err) => {
                            log::error!("Regex error: {}", err);
                            return Err(CliErrors::RegexPatternError);
                        }
                    }
                }
                false => {
                    if body.contains(search_pattern) {
                        counter += 1;
                    }
                }
            }
        }
        Ok(counter == text.len())
    }

    /// Matches patterns against the response body and returns matched patterns.
    pub fn match_once_body(
        &self,
        body: String,
        text: Vec<String>,
        is_regex: Option<bool>,
    ) -> Result<Vec<String>, CliErrors> {
        let mut matched_data = Vec::new();
        for pattern in text {
            match is_regex.unwrap_or(false) {
                true => {
                    match RegexBuilder::new(&pattern)
                        .multi_line(self.multi_line)
                        .case_insensitive(self.case_insensitive)
                        .unicode(self.unicode)
                        .octal(self.octal)
                        .dot_matches_new_line(self.dot_matches_new_line)
                        .build()
                    {
                        Ok(re) => {
                            if re.is_match(&body) {
                                matched_data.push(pattern);
                            }
                        }
                        Err(err) => {
                            log::error!("Regex error: {}", err);
                            return Err(CliErrors::RegexPatternError);
                        }
                    }
                }
                false => {
                    if body.contains(&pattern) {
                        matched_data.push(pattern);
                    }
                }
            }
        }
        Ok(matched_data)
    }
}

impl UserData for ResponseMatcher {
    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("is_match", |_, this, (regex_pattern, response): (String, String)| {
            this.is_match(regex_pattern, response).map_err(|e| e.to_lua_err())
        });

        methods.add_method("match_body", |_, this, (response, text_list, is_regex): (String, Vec<String>, Option<bool>)| {
            this.match_and_body(&response, text_list, is_regex).map_err(|e| e.to_lua_err())
        });

        methods.add_method("match_body_once", |_, this, (response, text_list, is_regex): (String, Vec<String>, Option<bool>)| {
            this.match_once_body(response, text_list, is_regex).map_err(|e| e.to_lua_err())
        });

        methods.add_method("replace", |_, this, (response, regex_pattern, replacement): (String, String, String)| {
            this.replace_txt(&regex_pattern, &replacement, &response).map_err(|e| e.to_lua_err())
        });

        methods.add_method("extract", |_, this, (regex_pattern, response): (String, String)| {
            this.extract_data(&regex_pattern, &response).map_err(|e| e.to_lua_err())
        });

        methods.add_method_mut("options", |_, this, opts: mlua::Table| {
            *this = ResponseMatcher {
                multi_line: opts.get("multi_line").unwrap_or(this.multi_line),
                case_insensitive: opts.get("case_insensitive").unwrap_or(this.case_insensitive),
                ignore_whitespace: opts.get("ignore_whitespace").unwrap_or(this.ignore_whitespace),
                unicode: opts.get("unicode").unwrap_or(this.unicode),
                octal: opts.get("octal").unwrap_or(this.octal),
                dot_matches_new_line: opts.get("dot_matches_new_line").unwrap_or(this.dot_matches_new_line),
            };
            Ok(())
        });
    }
}
