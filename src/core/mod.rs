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
use log::{debug, error};
use mlua::Lua;
use reqwest::header::HeaderMap;
use thirtyfour::prelude::*;

use utils::files::filename_to_string;
use utils::network::http as http_sender;
use utils::output::lua_report::report_script;
use utils::output::report::AllReports;
use utils::parsing::url::HttpMessage;
use utils::{get_matching_func, get_utilsfunc, http_func};

use std::fs::OpenOptions;
use std::io::Write;
use std::sync::Arc;

#[derive(Clone)]
pub struct RequestOpts {
    pub headers: HeaderMap,
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

        // Adding Lotus Lua Function
        get_utilsfunc(self.bar, &lua);
        get_matching_func(&lua);
        http_func(target_url, &lua);

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
                    self.request.headers.clone(),
                    self.request.proxy.clone(),
                    self.request.timeout,
                    self.request.redirects,
                ),
            )
            .unwrap();

        let main_func = lua.globals().get::<_, mlua::Function>("main").unwrap();
        debug!("MAIN FUNC STARTED");
        main_func
            .call_async::<_, mlua::Value>(target_url)
            .await
            .unwrap();
        debug!("MAIN FUNC DONE");

        if !report_code.is_empty() {
            // Still under development (not ready yet)
            report_script(filename_to_string(report_code).unwrap().as_str());
        } else {
            let final_report = lua.globals().get::<_, AllReports>("Reports");
            match final_report {
                Ok(the_report) => {
                    if !the_report.reports.is_empty() {
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
