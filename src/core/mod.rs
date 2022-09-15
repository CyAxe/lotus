pub mod utils;
use log::{debug, error, info, warn};
use mlua::Lua;
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::Write;
use std::sync::{Arc, Mutex};
use utils::html::{css_selector, html_parse, html_search};
use utils::is_match;
use utils::url::{change_urlquery, set_urlvalue, urljoin};

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
                    bar.println(format!("{} {}",console::Emoji("🔥","fire"),msg));
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
            .create_function(|_, url: (String, String)| Ok(change_urlquery(url.0, url.1)))
            .unwrap();

        lua.globals().set("log_info", log_info).unwrap();
        lua.globals().set("log_error", log_error).unwrap();
        lua.globals().set("log_debug", log_debug).unwrap();
        lua.globals().set("log_warn", log_warn).unwrap();
        lua.globals().set("change_urlquery", change_url).unwrap();
        lua.globals().set("set_urlvalue", set_urlvalue.unwrap()).unwrap();
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
    pub async fn run_scan(&self, script_code: &'a str, target_url: &'a str) -> mlua::Result<()> {
        let lua = self.get_activefunc();
        lua.globals().set("TARGET_URL",target_url).unwrap();

        lua.load(script_code)
            .exec_async()
            .await
            .unwrap();
        let out_table = lua.globals().get::<_,bool>("VALID".to_owned()).unwrap();
        if out_table == true {
                let out = lua.globals().get::<_, mlua::Table>("REPORT".to_owned()).unwrap();
                let new_report = Report {
                    url: out.get("url").unwrap(),
                    match_payload: out.get("match").unwrap(),
                    payload: out.get("payload").unwrap(),
                };
                let results = serde_json::to_string(&new_report).unwrap();
                self.write_report(&results);
        }
        self.bar.inc(1);
        Ok(())
    }
}
