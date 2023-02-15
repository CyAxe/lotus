/*
 * This file is part of Lotus Project, an Web Security Scanner written in Rust based on Lua Scripts
 * For details, please see https://github.com/rusty-sec/lotus/
 *
 * Copyright (c) 2022 - Khaled Nassar
 *
 * Please note that this file was originally released under the
 * GNU General Public License as ished by the Free Software Foundation;
 * either version 2 of the License, or (at your option) any later version.
 *
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use crate::{
    cli::{bar::BAR, errors::CliErrors},
    lua::{
        output::cve::CveReport,
        output::vuln::{AllReports, OutReport},
        parsing::{
            html::{css_selector, html_parse, html_search, Location},
            text::ResponseMatcher,
            url::HttpMessage,
        },
        payloads,
        threads::{LuaThreader, ParamScan},
    },
};

use log::{debug, error, info, warn};
use tokio::time::{sleep, Duration};

use console::Style;
use mlua::{ExternalError, Lua};
use url::Url;

use std::{
    fs::File,
    io::Read,
    path::Path,
    sync::{Arc, Mutex},
};

/// Setup The Lua Runtime
pub struct LuaRunTime<'lua> {
    pub lua: &'lua Lua,
}

impl LuaRunTime<'_> {
    /// * `target_url` - The Target URL
    pub fn setup(&self, target_url: Option<&str>) {
        self.http_func(target_url);
        self.payloads_func();
        self.get_utilsfunc();
        self.encoding_func();
        self.get_matching_func();
    }
    /// Setting up the payload functions
    pub fn payloads_func(&self) {
        self.lua
            .globals()
            .set(
                "XSSGenerator",
                self.lua
                    .create_function(
                        |_, (response, location, payload): (String, Location, String)| {
                            let xss_gen =
                                payloads::xss::PayloadGen::new(response, location, payload);
                            Ok(xss_gen.analyze())
                        },
                    )
                    .unwrap(),
            )
            .unwrap();
    }

    /// Setup the encoding function
    pub fn encoding_func(&self) {
        self.lua
            .globals()
            .set(
                "base64encode",
                self.lua
                    .create_function(|_, data: String| Ok(base64::encode(data)))
                    .unwrap(),
            )
            .unwrap();

        self.lua
            .globals()
            .set(
                "base64decode",
                self.lua
                    .create_function(|_, data: String| Ok(base64::decode(data).unwrap()))
                    .unwrap(),
            )
            .unwrap();
    }

    /// Setup the http functions
    pub fn http_func(&self, target_url: Option<&str>) {
        self.lua
            .globals()
            .set(
                "JOIN_SCRIPT_DIR",
                self.lua
                    .create_function(|c_lua, new_path: String| {
                        let script_path = c_lua.globals().get::<_, String>("SCRIPT_PATH").unwrap();
                        let the_path = Path::new(&script_path);
                        Ok(the_path
                            .parent()
                            .unwrap()
                            .join(new_path)
                            .to_str()
                            .unwrap()
                            .to_string())
                    })
                    .unwrap(),
            )
            .unwrap();

        let log_info = self
            .lua
            .create_function(|_, log_msg: String| {
                info!("{}", log_msg);
                Ok(())
            })
            .unwrap();
        let log_warn = self
            .lua
            .create_function(|_, log_msg: String| {
                warn!("{}", log_msg);
                Ok(())
            })
            .unwrap();
        let log_debug = self
            .lua
            .create_function(|_, log_msg: String| {
                debug!("{}", log_msg);
                Ok(())
            })
            .unwrap();
        let log_error = self
            .lua
            .create_function(|_, log_msg: String| {
                error!("{}", log_msg);
                Ok(())
            })
            .unwrap();
        self.lua
            .globals()
            .set(
                "sleep",
                self.lua
                    .create_async_function(|_, time: u64| async move {
                        sleep(Duration::from_secs(time)).await;
                        Ok(())
                    })
                    .unwrap(),
            )
            .unwrap();

        self.lua
            .globals()
            .set(
                "readfile",
                self.lua
                    .create_function(|_ctx, file_path: String| {
                        if Path::new(&file_path).exists() {
                            let mut file = File::open(&file_path)?;
                            let mut file_content = String::new();
                            file.read_to_string(&mut file_content)?;
                            Ok(file_content)
                        } else {
                            Err(CliErrors::ReadingError.to_lua_err())
                        }
                    })
                    .unwrap(),
            )
            .unwrap();
        self.lua.globals().set("log_info", log_info).unwrap();
        self.lua.globals().set("log_error", log_error).unwrap();
        self.lua.globals().set("log_debug", log_debug).unwrap();
        self.lua.globals().set("log_warn", log_warn).unwrap();

        if !target_url.is_none() {
            self.lua
                .globals()
                .set(
                    "HttpMessage",
                    HttpMessage {
                        url: Url::parse(target_url.unwrap()).unwrap(),
                    },
                )
                .unwrap();
        }
        self.lua
            .globals()
            .set(
                "Reports",
                AllReports {
                    reports: Vec::new(),
                },
            )
            .unwrap();
        self.lua
            .globals()
            .set("VulnReport", OutReport::init())
            .unwrap();
        self.lua
            .globals()
            .set("CveReport", CveReport::init())
            .unwrap();
    }
    /// Setup Lotus utils functions
    pub fn get_utilsfunc<'prog, 'lua>(&self) {
        // ProgressBar
        self.lua
            .globals()
            .set("ResponseMatcher", ResponseMatcher {})
            .unwrap();
        self.lua
            .globals()
            .set(
                "ParamScan",
                ParamScan {
                    finds: Arc::new(Mutex::new(false)),
                    accept_nil: Arc::new(Mutex::new(false)),
                },
            )
            .unwrap();
        self.lua
            .globals()
            .set(
                "LuaThreader",
                LuaThreader {
                    stop: Arc::new(Mutex::new(false)),
                },
            )
            .unwrap();
        self.lua
            .globals()
            .set(
                "str_startswith",
                self.lua
                    .create_function(|_, (str_one, str_two): (String, String)| {
                        Ok(str_one.starts_with(&str_two))
                    })
                    .unwrap(),
            )
            .unwrap();
        self.lua
            .globals()
            .set(
                "str_contains",
                self.lua
                    .create_function(|_, (str_one, str_two): (String, String)| {
                        Ok(str_one.contains(&str_two))
                    })
                    .unwrap(),
            )
            .unwrap();
        self.lua
            .globals()
            .set(
                "print_cve_report",
                self.lua
                    .create_function(move |_, the_report: CveReport| {
                        let good_msg = format!("[{}]", Style::new().green().apply_to("+"));
                        let info_msg = format!("[{}]", Style::new().blue().apply_to("#"));
                        let report_msg = format!(
                            "
{GOOD} {NAME} on: {URL}
{INFO} SCAN TYPE: CVE
{INFO} Description: {Description}
{INFO} Risk: {RISK}
{INFO} Matching Pattern: {MATCHING}
#--------------------------------------------------#

                                     ",
                            GOOD = good_msg,
                            INFO = info_msg,
                            NAME = the_report.name.unwrap(),
                            URL = the_report.url.unwrap(),
                            Description = the_report.description.unwrap(),
                            RISK = the_report.risk.unwrap(),
                            MATCHING = format!("{:?}", the_report.matchers),
                        );
                        {
                            BAR.lock().unwrap().println(report_msg)
                        };
                        Ok(())
                    })
                    .unwrap(),
            )
            .unwrap();
        self.lua
            .globals()
            .set(
                "print_vuln_report",
                self.lua
                    .create_function(move |_, the_report: OutReport| {
                        let good_msg = format!("[{}]", Style::new().green().apply_to("+"));
                        let info_msg = format!("[{}]", Style::new().blue().apply_to("#"));
                        let report_msg = format!(
                            "
{GOOD} {NAME} on: {URL}
{INFO} SCAN TYPE: VULN
{INFO} Description: {Description}
{INFO} Vulnerable Parameter: {PARAM}
{INFO} Risk: {RISK}
{INFO} Used Payload: {ATTACK}
{INFO} Matching Pattern: {MATCHING}
#--------------------------------------------------#

                                     ",
                            GOOD = good_msg,
                            INFO = info_msg,
                            NAME = the_report.name.unwrap(),
                            URL = the_report.url.unwrap(),
                            Description = the_report.description.unwrap(),
                            PARAM = the_report.param.unwrap(),
                            RISK = the_report.risk.unwrap(),
                            ATTACK = the_report.attack.unwrap(),
                            MATCHING = format!(
                                "{}",
                                Style::new().on_red().apply_to(the_report.evidence.unwrap())
                            ),
                        );
                        {
                            BAR.lock().unwrap().println(report_msg)
                        };
                        Ok(())
                    })
                    .unwrap(),
            )
            .unwrap();

        self.lua
            .globals()
            .set(
                "println",
                self.lua
                    .create_function(move |_, msg: String| {
                        {
                            BAR.lock().unwrap().println(&msg)
                        };
                        Ok(())
                    })
                    .unwrap(),
            )
            .unwrap();
    }

    pub fn get_matching_func(&self) {
        // Regex Match
        self.lua
            .globals()
            .set(
                "is_match",
                self.lua
                    .create_function(|_, (pattern, text): (String, String)| {
                        let try_match = is_match(pattern, text);
                        if try_match.is_err() {
                            Err(try_match.unwrap_err().to_lua_err())
                        } else {
                            Ok(try_match.unwrap())
                        }
                    })
                    .unwrap(),
            )
            .unwrap();
        self.lua
            .globals()
            .set(
                "generate_css_selector",
                self.lua
                    .create_function(|_, payload: String| Ok(css_selector(&payload)))
                    .unwrap(),
            )
            .unwrap();

        self.lua
            .globals()
            .set(
                "html_parse",
                self.lua
                    .create_function(|_, (html, payload): (String, String)| {
                        Ok(html_parse(&html, &payload))
                    })
                    .unwrap(),
            )
            .unwrap();

        self.lua
            .globals()
            .set(
                "html_search",
                self.lua
                    .create_function(|_, (html, pattern): (String, String)| {
                        Ok(html_search(&html, &pattern))
                    })
                    .unwrap(),
            )
            .unwrap();
    }
}

/// check if the regex pattern is matching with this string or not without get the matched parts
pub fn is_match(pattern: String, resp: String) -> Result<bool, CliErrors> {
    let re = fancy_regex::Regex::new(&pattern);
    if let Ok(..) = re {
        let matched = re.unwrap().is_match(&resp);
        if matched.is_err() {
            error!("Cannot match with resp value: {}", resp);
            Err(CliErrors::RegexError)
        } else {
            Ok(matched.unwrap())
        }
    } else {
        error!("Regex Pattern ERROR  {:?}", pattern);
        Err(CliErrors::RegexPatternError)
    }
}
