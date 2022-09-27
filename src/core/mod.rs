pub mod utils;
use futures::{stream, StreamExt};
use log::{debug, error, info, warn};
use mlua::Lua;
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::Write;
use std::sync::{Arc, Mutex};
use thirtyfour::prelude::*;
use utils::html::{css_selector, html_parse, html_search};
use utils::is_match;
use utils::url::{change_urlquery, set_urlvalue, urljoin};
use std::fs::File;
use std::io::Read;


#[derive(Clone)]
pub struct LuaLoader<'a> {
    output_dir: Arc<Mutex<String>>,
    bar: &'a indicatif::ProgressBar,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Report {
    pub payload: String,
    pub match_payload: String,
    pub url: String,
}

impl<'a> LuaLoader<'a> {
    pub fn new(bar: &'a indicatif::ProgressBar, output_dir: String) -> LuaLoader {
        LuaLoader {
            output_dir: Arc::new(Mutex::new(output_dir)),
            bar,
        }
    }

    fn write_report(&self, results: &str) {
        let out_dir = self.output_dir.lock().unwrap();
        OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open(out_dir.to_string())
            .expect("Could not open file")
            .write_all(format!("{}\n", results).as_str().as_bytes())
            .expect("Could not write to file");
    }
    fn get_activefunc(&self) -> Lua {
        let lua = Lua::new();
        lua.globals()
            .set(
                "welcome",
                lua.create_function(|_, name: String| {
                    println!("WELCOME {}", name);
                    Ok(())
                })
                .unwrap(),
            )
            .unwrap();
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
                "send_req",
                lua.create_async_function(|_, url: String| async move {
                    let mut sender = utils::http::Sender::init();
                    Ok(sender.send(url).await)
                })
                .unwrap(),
            )
            .unwrap();
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
                "sleep",
                lua.create_function(|_, time: u64| {
                    std::thread::sleep(std::time::Duration::from_secs(time));
                    Ok(())
                })
                .unwrap(),
            )
            .unwrap();
        // ProgressBar
        let bar = self.bar.clone();
        lua.globals()
            .set(
                "println",
                lua.create_function(move |_, msg: String| {
                    bar.println(format!("{} {}", console::Emoji("🔥", "fire"), msg));
                    Ok(())
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

        // Set Functions
        let set_urlvalue =
            lua.create_function(|_, (url, param, payload): (String, String, String)| {
                Ok(set_urlvalue(&url, &param, &payload))
            });
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
        let change_url = lua
            .create_function(
                |_, (url, payload, remove_content): (String, String, bool)| {
                    Ok(change_urlquery(url, payload, remove_content))
                },
            )
            .unwrap();

        lua.globals().set("log_info", log_info).unwrap();
        lua.globals().set("log_error", log_error).unwrap();
        lua.globals().set("log_debug", log_debug).unwrap();
        lua.globals().set("log_warn", log_warn).unwrap();
        lua.globals().set("change_urlquery", change_url).unwrap();
        lua.globals()
            .set("set_urlvalue", set_urlvalue.unwrap())
            .unwrap();
        lua.globals()
            .set(
                "generate_css_selector",
                lua.create_function(|_, html: String| Ok(css_selector(&html)))
                    .unwrap(),
            )
            .unwrap();

        lua.globals()
            .set(
                "urljoin",
                lua.create_function(|_, (url, path): (String, String)| Ok(urljoin(url, path)))
                    .unwrap(),
            )
            .unwrap();
        lua.globals()
            .set("read",
                 lua.create_function(|_ctx, file_path: String| {
                    use std::path::Path;
                    if Path::new(&file_path).exists() == true {
                        let mut file = File::open(&file_path)?;
                        let mut file_content = String::new();
                        file.read_to_string(&mut file_content)?;
                        Ok(file_content)
                    } else {
                        Ok(0.to_string())
                    }
                 }).unwrap()
                 ).unwrap();

        lua.globals()
            .set(
                "html_search",
                lua.create_function(|_, (html, pattern): (String, String)| {
                    Ok(html_search(&html, &pattern))
                })
                .unwrap(),
            )
            .unwrap();

        lua
    }
    pub async fn run_scan(
        &self,
        driver: Option<Arc<Mutex<WebDriver>>>,
        script_code: &'a str,
        script_dir: &'a str,
        target_url: &'a str,
    ) -> mlua::Result<()> {
        let lua = self.get_activefunc();
        lua.globals().set("TARGET_URL", target_url).unwrap();
        match driver {
            None => {}
            _ => {
                lua.globals()
                    .set(
                        "open",
                        lua.create_function(move |_, url: String| {
                            futures::executor::block_on({
                                let driver = Arc::clone(&driver.as_ref().unwrap());
                                async move {
                                    driver.lock().unwrap().goto(url).await.unwrap();
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
                .pairs::<String, String>()
                .into_iter()
                .for_each(|item| {
                    all_payloads.push(item);
                });
            all_payloads
        };
        let main_func = lua.globals().get::<_, mlua::Function>("main").unwrap();
        let script_threads = lua.globals().get::<_, usize>("THREADS").unwrap();
        let globals = lua.globals();
        let reports = stream::iter(payloads.into_iter())
            .map(move |payload| {
                let globals = globals.clone();
                let main_func = main_func.clone();
                async move {
                    if globals.get::<_, bool>("STOP_AFTER_MATCH").unwrap() == true {
                        if globals.get::<_, bool>("VALID").unwrap() == false {
                            main_func
                                .call_async::<_, mlua::Table>(payload.unwrap())
                                .await
                                .unwrap();
                        }
                    } else {
                        main_func
                            .call_async::<_, mlua::Table>(payload.unwrap())
                            .await
                            .unwrap();
                    }
                    globals.get::<_, mlua::Table>("REPORT").unwrap()
                }
            })
            .buffer_unordered(script_threads)
            .collect::<Vec<_>>()
            .await;
        let out_table = lua.globals().get::<_, bool>("VALID".to_owned()).unwrap();
        if out_table == true {
            reports.iter().for_each(|out| {
                let new_report = Report {
                    url: out.get("url").unwrap_or("".into()),
                    match_payload: out.get("match").unwrap_or("".into()),
                    payload: out.get("payload").unwrap_or("".to_string()),
                };
                let results = serde_json::to_string(&new_report).unwrap();
                self.write_report(&results);
            });
        }
        self.bar.inc(1);
        Ok(())
    }
}
