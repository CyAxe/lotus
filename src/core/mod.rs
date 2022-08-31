pub mod utils;
use rlua::Lua;
use tracing::{debug, error, info, warn};

pub struct LuaLoader {
    bar: indicatif::ProgressBar,
}

impl LuaLoader {
    pub fn new(bar: indicatif::ProgressBar) -> LuaLoader {
        LuaLoader { bar }
    }

    pub fn run_scan(&self, script_code: &str, target_url: &str) {
        let lua_code = Lua::new();
        let sender = utils::Sender::init();
        lua_code.context(|lua_context| {
            let global = lua_context.globals();
            // Set Functions
            let set_urlvalue = lua_context.create_function(
                |_, (url, param, payload): (String, String, String)| {
                    Ok(utils::set_urlvalue(&url, &param, &payload))
                },
            );
            let log_info = lua_context
                .create_function(|_, log_msg: String| {
                    info!("{}", log_msg);
                    Ok(())
                })
                .unwrap();
            let log_warn = lua_context
                .create_function(|_, log_msg: String| {
                    warn!("{}", log_msg);
                    Ok(())
                })
                .unwrap();
            let log_debug = lua_context
                .create_function(|_, log_msg: String| {
                    debug!("{}", log_msg);
                    Ok(())
                })
                .unwrap();
            let log_error = lua_context
                .create_function(|_, log_msg: String| {
                    error!("{}", log_msg);
                    Ok(())
                })
                .unwrap();
            let change_url = lua_context
                .create_function(|_, url: (String, String)| {
                    Ok(utils::change_urlquery(url.0, url.1))
                })
                .unwrap();
            let send_req_func = lua_context
                .create_function(move |_, url: String| Ok(sender.send(url)))
                .unwrap();

            // Set Globals
            global.set("log_info", log_info).unwrap();
            global.set("log_error", log_error).unwrap();
            global.set("log_debug", log_debug).unwrap();
            global.set("log_warn", log_warn).unwrap();
            global.set("change_urlquery", change_url).unwrap();
            global.set("set_urlvalue", set_urlvalue.unwrap()).unwrap();
            global.set("send_req", send_req_func).unwrap();
            global
                .set(
                    "is_match",
                    lua_context
                        .create_function(|_, (pattern, body): (String, String)| {
                            Ok(utils::is_match(pattern, body))
                        })
                        .unwrap(),
                )
                .unwrap();
            global
                .set(
                    "show_bug",
                    lua_context
                        .create_function(|_, _report_data: rlua::Table| Ok(()))
                        .unwrap(),
                )
                .unwrap();

            global
                .set(
                    "html_parse",
                    lua_context
                        .create_function(|_, (html, payload): (String, String)| {
                            Ok(utils::html_parse(&html, &payload))
                        })
                        .unwrap(),
                )
                .unwrap();

            global
                .set(
                    "generate_css_selector",
                    lua_context
                        .create_function(|_, html: String| Ok(utils::css_selector(&html)))
                        .unwrap(),
                )
                .unwrap();

            global
                .set(
                    "html_search",
                    lua_context
                        .create_function(|_, (html, pattern): (String, String)| {
                            Ok(utils::html_search(&html, &pattern))
                        })
                        .unwrap(),
                )
                .unwrap();

            // Execute The Script
            lua_context.load(script_code).exec().unwrap();
            let main_func: rlua::Function = global.get("main").unwrap();
            let out = main_func.call::<_, rlua::Table>(target_url).unwrap();
            out.pairs::<String, String>().for_each(|_d| {
                debug!("HACKER MAN : {:?}", _d);
            });
        });
        self.bar.inc(1);
    }
}
