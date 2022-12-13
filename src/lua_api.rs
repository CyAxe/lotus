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

use crate::parsing::html::{css_selector, html_parse, html_search, Location};
use crate::parsing::url::HttpMessage;
use crate::payloads;
use crate::output::report::{OutReport, AllReports};

use console::Style;
use log::{debug, error, info, warn};
use mlua::Lua;
use tokio::time::{sleep, Duration};
use url::Url;

use std::fs::File;
use std::io::Read;
use std::path::Path;

/// check if the regex pattern is matching with this string or not without get the matched parts
/// you can use it for sqli errors for example
/// ```rust
/// assert_eq!(true,is_match("\d","1"));
/// assert_eq!(false,is_match("\d\d","1"))
/// ```
pub fn is_match(pattern: String, resp: String) -> bool {
    let re = fancy_regex::Regex::new(&pattern);
    if let Ok(..) = re {
        re.unwrap().is_match(&resp).unwrap_or(false)
    } else {
        debug!("MATCHING ERROR  {:?} | {:?}", pattern, re);
        false
    }
}

pub fn payloads_func(lua: &Lua) {
    lua.globals().set("XSSGenerator", lua.create_function(|_, (response, location, payload): (String, Location, String)| {
        let xss_gen = payloads::xss::PayloadGen::new(response, location, payload);
        Ok(xss_gen.analyze())
    }).unwrap()).unwrap();
}

pub fn encoding_func(lua: &Lua) {
    lua.globals().set("base64encode", lua.create_function(|_, data: String| {
        Ok(base64::encode(data))
    }).unwrap()).unwrap();

    lua.globals().set("base64decode", lua.create_function(|_, data: String| {
        Ok(base64::decode(data).unwrap())
    }).unwrap()).unwrap();
}

pub fn http_func(target_url: &str, lua: &Lua) {
    lua.globals()
        .set(
            "HttpMessage",
            HttpMessage {
                url: Url::parse(target_url).unwrap(),
            },
        )
        .unwrap();
    lua.globals()
        .set(
            "Reports",
            AllReports {
                reports: Vec::new(),
            },
        )
        .unwrap();
    lua.globals().set("NewReport", OutReport::init()).unwrap();
}

pub fn get_utilsfunc<'prog>(the_bar: &'prog indicatif::ProgressBar, lua: &Lua) {
    // ProgressBar
    let bar = the_bar.clone();
    lua.globals()
        .set(
            "print_report",
            lua.create_function(move |_, the_report: OutReport| {
                let good_msg = format!("[{}]", Style::new().green().apply_to("+"));
                let info_msg = format!("[{}]", Style::new().blue().apply_to("#"));
                let report_msg = format!(
                    "
{GOOD} {NAME} on: {URL}
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
                bar.println(report_msg);
                Ok(())
            })
            .unwrap(),
        )
        .unwrap();

    let bar = the_bar.clone();
    lua.globals()
        .set(
            "println",
            lua.create_function(move |_, msg: String| {
                bar.println(&msg);
                Ok(())
            })
            .unwrap(),
        )
        .unwrap();

    let log_info = lua
        .create_function(|_, log_msg: String| {
            info!("{}", log_msg);
            Ok(())
        })
        .unwrap();
    let log_warn = lua
        .create_function(|_, log_msg: String| {
            warn!("{}", log_msg);
            Ok(())
        })
        .unwrap();
    let log_debug = lua
        .create_function(|_, log_msg: String| {
            debug!("{}", log_msg);
            Ok(())
        })
        .unwrap();
    let log_error = lua
        .create_function(|_, log_msg: String| {
            error!("{}", log_msg);
            Ok(())
        })
        .unwrap();
    lua.globals()
        .set(
            "sleep",
            lua.create_async_function(|_, time: u64| async move {
                sleep(Duration::from_secs(time)).await;
                Ok(())
            })
            .unwrap(),
        )
        .unwrap();

    lua.globals()
        .set(
            "read",
            lua.create_function(|_ctx, file_path: String| {
                if Path::new(&file_path).exists() {
                    let mut file = File::open(&file_path)?;
                    let mut file_content = String::new();
                    file.read_to_string(&mut file_content)?;
                    Ok(file_content)
                } else {
                    Ok(0.to_string())
                }
            })
            .unwrap(),
        )
        .unwrap();
    lua.globals().set("log_info", log_info).unwrap();
    lua.globals().set("log_error", log_error).unwrap();
    lua.globals().set("log_debug", log_debug).unwrap();
    lua.globals().set("log_warn", log_warn).unwrap();
}

pub fn get_matching_func(lua: &Lua) {
    // Regex Match
    lua.globals()
        .set(
            "is_match",
            lua.create_function(|_, (pattern, text): (String, String)| Ok(is_match(pattern, text)))
                .unwrap(),
        )
        .unwrap();
    lua.globals()
        .set(
            "generate_css_selector",
            lua.create_function(|_, payload: String| Ok(css_selector(&payload)))
                .unwrap(),
        )
        .unwrap();

    lua.globals()
        .set(
            "html_parse",
            lua.create_function(|_, (html, payload): (String, String)| {
                Ok(html_parse(&html, &payload))
            })
            .unwrap(),
        )
        .unwrap();

    lua.globals()
        .set(
            "html_search",
            lua.create_function(|_, (html, pattern): (String, String)| {
                Ok(html_search(&html, &pattern))
            })
            .unwrap(),
        )
        .unwrap();
}
