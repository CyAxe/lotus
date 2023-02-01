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
use serde::{Deserialize, Serialize};

#[derive(Clone,Debug, Deserialize, Serialize)]
pub enum ReportMatchers {
    RawResponse(String),
    ResposneHeaders(String),
    ResponseBody(String),
    StatusCode(String),
    General(String),
}

#[derive(Clone, Deserialize, Serialize)]
pub struct CveReport {
    pub name: Option<String>,
    pub description: Option<String>,
    pub url: Option<String>,
    pub risk: Option<String>,
    pub matchers: Vec<ReportMatchers>,
}

impl UserData for CveReport {
    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method_mut("setRisk", |_, this, risk: String| {
            this.risk = Some(risk);
            Ok(())
        });
        methods.add_method_mut("setName", |_, this, name: String| {
            this.name = Some(name);
            Ok(())
        });
        methods.add_method_mut("setUrl", |_, this, url: String| {
            this.url = Some(url);
            Ok(())
        });

        methods.add_method_mut("setDescription", |_, this, description: String| {
            this.description = Some(description);
            Ok(())
        });

        methods.add_method_mut(
            "addMatcher",
            |_, this, (matcher_string, matching_type): (String, i32)| {
                let matcher_data = match matching_type {
                    1 => ReportMatchers::RawResponse(matcher_string),
                    2 => ReportMatchers::ResposneHeaders(matcher_string),
                    3 => ReportMatchers::ResponseBody(matcher_string),
                    4 => ReportMatchers::StatusCode(matcher_string),
                    _ => ReportMatchers::General(matcher_string),
                };
                this.matchers.push(matcher_data);
                Ok(())
            },
        );
    }
}

impl CveReport {
    pub fn init() -> CveReport {
        CveReport {
            name: None,
            description: None,
            url: None,
            risk: None,
            matchers: vec![],
        }
    }
}
