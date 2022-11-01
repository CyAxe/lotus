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

pub mod utils;
use futures::lock::Mutex;
use log::{debug, error, info, warn};
use mlua::Lua;
use thirtyfour::prelude::*;
use url::Url;

use utils::files::filename_to_string;
use utils::html::{css_selector, html_parse, html_search};
use utils::http as http_sender;
use utils::is_match;
use utils::lua_report::report_script;
use utils::report::{AllReports,OutReport};
use utils::url::HttpMessage;

use std::fs::File;
use std::fs::OpenOptions;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::sync::Arc;

#[derive(Clone)]
pub struct RequestOpts {
    pub proxy: Option<String>,
    pub timeout: u64,
    pub redirects: u32,
}

#[derive(Clone)]
pub struct LuaLoader<'a> {
    output_dir: String,
    request: RequestOpts,
    bar: &'a indicatif::ProgressBar,
}

/// Start Lotus by adding the ProgressBar and http request options
impl<'a> LuaLoader<'a> {
    pub fn new(
        bar: &'a indicatif::ProgressBar,
        request: RequestOpts,
        output_dir: String,
    ) -> LuaLoader {
        LuaLoader {
            output_dir,
            request,
            bar,
        }
    }

    fn write_report(&self, results: &str) {
        OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open(&self.output_dir)
            .expect("Could not open file")
            .write_all(format!("{}\n", results).as_str().as_bytes())
            .expect("Could not write to file");
    }
    fn get_matching_func(&self, lua: &Lua) {
        // Regex Match
        lua.globals()
            .set(
                "is_match",
                lua.create_function(|_, (pattern, text): (String, String)| {
                    Ok(is_match(pattern, text))
                })
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

    fn get_utilsfunc(&self, lua: &Lua) {
        // ProgressBar
        let bar = self.bar.clone();
        lua.globals()
            .set(
                "println",
                lua.create_function(move |_, msg: String| {
                    bar.println(format!("{}", msg));
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

    fn get_httpfunc(&self, lua: &Lua) {
        lua.globals()
            .set(
                "sleep",
                lua.create_async_function(|_, time: u64| async move {
                    tokio::time::sleep(tokio::time::Duration::from_secs(time)).await;
                    Ok(())
                })
                .unwrap(),
            )
            .unwrap();
    }

    /// Start All Lua Scripts
    pub async fn run_scan(
        &self,
        driver: Option<Arc<Mutex<WebDriver>>>,
        script_code: &'a str,
        script_dir: &'a str,
        target_url: &'a str,
        report_code: &'a str,
    ) -> mlua::Result<()> {
        let lua = Lua::new();
        self.get_httpfunc(&lua);
        self.get_utilsfunc(&lua);
        self.get_matching_func(&lua);
        lua.globals()
            .set(
                "HttpMessage",
                HttpMessage {
                    url: Url::parse(target_url).unwrap(),
                },
            )
            .unwrap();
        lua.globals()
            .set("Reports",AllReports {
                reports: Vec::new()
            }).unwrap();
        lua.globals()
            .set("NewReport", OutReport::init()).unwrap();
        match driver {
            None => {}
            _ => {
                lua.globals()
                    .set(
                        "open",
                        lua.create_function(move |_, url: String| {
                            futures::executor::block_on({
                                let driver = Arc::clone(driver.as_ref().unwrap());
                                async move {
                                    driver.lock().await.goto(url).await.unwrap();
                                }
                            });
                            Ok(())
                        })
                        .unwrap(),
                    )
                    .unwrap();
            }
        };
        lua.globals().set("SCRIPT_PATH", script_dir).unwrap();

        lua.load(script_code).exec_async().await.unwrap();

        // HTTP Sender
        lua.globals()
            .set(
                "http",
                http_sender::Sender::init(
                    self.request.proxy.clone(),
                    self.request.timeout,
                    self.request.redirects,
                ),
            )
            .unwrap();

        let main_func = lua.globals().get::<_, mlua::Function>("main").unwrap();
        debug!("MAIN FUNC STARTED");
        main_func.call_async::<_, mlua::Value>(target_url).await.unwrap();
        debug!("MAIN FUNC DONE");

        if report_code.len() > 0 {
            // Still under development (not ready yet)
            report_script(filename_to_string(report_code).unwrap().as_str());
        } else {
            let final_report = lua.globals().get::<_, AllReports>("Reports");
            match final_report {
                Ok(the_report) => {
                    if the_report.clone().reports.len() > 0 {
                        let results = serde_json::to_string(&the_report.reports).unwrap();
                        self.write_report(&results);
                    }
                }
                Err(err) => {
                    error!("Report Error: {}", err);
                }
            }
        }
        self.bar.inc(1);
        Ok(())
    }
}
