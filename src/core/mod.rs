pub mod utils;
use futures::executor::block_on;
use log::{debug, error, info, warn};
use rlua::Lua;
use rlua_async::{ChunkExt, ContextExt};
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::Write;
use utils::html::{css_selector, html_parse, html_search};
use utils::is_match;
use utils::url::{change_urlquery, set_urlvalue, urljoin};

pub struct LuaLoader {}

#[derive(Serialize, Deserialize, Debug)]
pub struct Report {
    pub payload: String,
    pub match_payload: String,
    pub url: String,
}

impl<'a> LuaLoader {
    pub fn new() -> LuaLoader {
        LuaLoader {}
    }

    fn write_report(&self, output_dir: &str, results: &str) {
        OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open(output_dir)
            .expect("Could not open file")
            .write_all(format!("{}\n", results).as_str().as_bytes())
            .expect("Could not write to file");
    }
    pub fn run_scan(
        &self,
        bar: &'a indicatif::ProgressBar,
        output_dir: &str,
        script_code: &str,
        target_url: &str,
    ) -> rlua::Result<()> {
        let lua = Lua::new();
        let sender = utils::http::Sender::init();
        lua.context(move |ctx| {
            let globals = ctx.globals();
            // Regex Match
            globals
                .set(
                    "is_match",
                    ctx.create_function(|_, (pattern, text): (String, String)| {
                        Ok(is_match(pattern, text))
                    })
                    .unwrap(),
                )
                .unwrap();
            // ProgressBar
            let bar = bar.clone();
            globals
                .set(
                    "println",
                    ctx.create_function(move |_, msg: String| {
                        bar.println(msg);
                        Ok(())
                    })
                    .unwrap(),
                )
                .unwrap();
            globals
                .set(
                    "generate_css_selector",
                    ctx.create_function(|_, payload: String| Ok(css_selector(&payload)))
                        .unwrap(),
                )
                .unwrap();

            globals
                .set(
                    "html_parse",
                    ctx.create_function(|_, (html, payload): (String, String)| {
                        Ok(html_parse(&html, &payload))
                    })
                    .unwrap(),
                )
                .unwrap();

            // Set Functions
            let set_urlvalue =
                ctx.create_function(|_, (url, param, payload): (String, String, String)| {
                    Ok(set_urlvalue(&url, &param, &payload))
                });
            let log_info = ctx
                .create_function(|_, log_msg: String| {
                    info!("{}", log_msg);
                    Ok(())
                })
                .unwrap();
            let log_warn = ctx
                .create_function(|_, log_msg: String| {
                    warn!("{}", log_msg);
                    Ok(())
                })
                .unwrap();
            let log_debug = ctx
                .create_function(|_, log_msg: String| {
                    debug!("{}", log_msg);
                    Ok(())
                })
                .unwrap();
            let log_error = ctx
                .create_function(|_, log_msg: String| {
                    error!("{}", log_msg);
                    Ok(())
                })
                .unwrap();
            let change_url = ctx
                .create_function(|_, url: (String, String)| Ok(change_urlquery(url.0, url.1)))
                .unwrap();

            globals.set("log_info", log_info).unwrap();
            globals.set("log_error", log_error).unwrap();
            globals.set("log_debug", log_debug).unwrap();
            globals.set("log_warn", log_warn).unwrap();
            globals.set("change_urlquery", change_url).unwrap();
            globals.set("set_urlvalue", set_urlvalue.unwrap()).unwrap();
            globals
                .set(
                    "generate_css_selector",
                    ctx.create_function(|_, html: String| Ok(css_selector(&html)))
                        .unwrap(),
                )
                .unwrap();

            globals
                .set(
                    "urljoin",
                    ctx.create_function(|_, (url, path): (String, String)| Ok(urljoin(url, path)))
                        .unwrap(),
                )
                .unwrap();

            globals
                .set(
                    "html_search",
                    ctx.create_function(|_, (html, pattern): (String, String)| {
                        Ok(html_search(&html, &pattern))
                    })
                    .unwrap(),
                )
                .unwrap();
            globals.set(
                "send_req",
                ctx.create_async_function(move |_ctx, url: String| {
                    let mut sender = sender.clone();
                    async move {
                        let resp = sender.send(url).await;
                        Ok(resp)
                    }
                })
                .unwrap(),
            )
        })?;
        lua.context(|ctx| {
            let global = ctx.globals();
            global.set("TARGET_URL", target_url)
        })?;
        lua.context(|ctx| {
            let chunk = ctx.load(script_code);
            block_on(chunk.exec_async(ctx))
        })?;

        lua.context(|ctx| {
            let global = ctx.globals();
            if global.get::<_, bool>("valid".to_owned()).unwrap() == true {
                println!("TRUE");
            }

            let out = global.get::<_, rlua::Table>("report".to_owned()).unwrap();
            let new_report = Report {
                url: out.get("url").unwrap(),
                match_payload: out.get("match").unwrap(),
                payload: out.get("payload").unwrap(),
            };
            let results = serde_json::to_string(&new_report).unwrap();
            self.write_report(output_dir, &results);
        });
        bar.inc(1);
        Ok(())
    }
}
