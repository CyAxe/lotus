pub mod utils;
use futures::{lock::Mutex, stream, StreamExt};
use log::{debug, error, info, warn};
use mlua::Lua;
use thirtyfour::prelude::*;

use utils::html::{css_selector, html_parse, html_search};
use utils::http as http_sender;
use utils::is_match;
use utils::url::{change_urlquery, set_urlvalue, urljoin};
use utils::report::report_script;
use utils::files::filename_to_string;

use std::collections::HashMap;
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
                "set_urlvalue",
                lua.create_function(|_, (url, param, payload): (String, String, String)| {
                    Ok(set_urlvalue(&url, &param, &payload))
                })
                .unwrap(),
            )
            .unwrap();

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
        // Set Functions
        let set_urlvalue =
            lua.create_function(|_, (url, param, payload): (String, String, String)| {
                Ok(set_urlvalue(&url, &param, &payload))
            });
        let change_url = lua
            .create_function(
                |_, (url, payload, remove_content): (String, String, bool)| {
                    Ok(change_urlquery(url, payload, remove_content))
                },
            )
            .unwrap();
        lua.globals().set("change_urlquery", change_url).unwrap();
        lua.globals()
            .set("set_urlvalue", set_urlvalue.unwrap())
            .unwrap();

        lua.globals()
            .set(
                "urljoin",
                lua.create_function(|_, (url, path): (String, String)| Ok(urljoin(url, path)))
                    .unwrap(),
            )
            .unwrap();
    }

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
        lua.globals().set("TARGET_URL", target_url).unwrap();
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
        let payloads_func = lua
            .globals()
            .get::<_, mlua::Function>("payloads_gen")
            .unwrap();
        let payloads = payloads_func
            .call_async::<_, mlua::Table>(target_url)
            .await
            .unwrap();
        let payloads = {
            let mut all_payloads = Vec::new();
            payloads
                .pairs::<mlua::Value, mlua::Value>()
                .into_iter()
                .for_each(|item| {
                    all_payloads.push(item.unwrap());
                });
            all_payloads
        };
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
        let script_threads = lua.globals().get::<_, usize>("THREADS").unwrap_or(5);
        stream::iter(payloads.into_iter())
            .map(move |payload| {
                let main_func = main_func.clone();
                async move {
                    main_func
                        .call_async::<_, mlua::Table>(payload)
                        .await
                        .unwrap();
                }
            })
            .buffer_unordered(script_threads)
            .collect::<Vec<_>>()
            .await;
        if report_code.len() > 0 {
            report_script(filename_to_string(report_code).unwrap().as_str());
        } else {
            let out_table = lua.globals().get::<_, bool>("VALID".to_owned()).unwrap();
            if out_table {
                let mut test_report: HashMap<String, mlua::Value> = HashMap::new();
                lua.globals()
                    .get::<_, mlua::Table>("REPORT")
                    .unwrap()
                    .pairs::<String, mlua::Value>()
                    .for_each(|out_report| {
                        let current_out = out_report.clone();
                        test_report.insert(current_out.unwrap().0, out_report.unwrap().1);
                    });
                let results = serde_json::to_string(&test_report).unwrap();
                self.write_report(&results);
            }
        }
        self.bar.inc(1);
        Ok(())
    }
}
