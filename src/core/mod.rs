pub mod utils;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use rlua::Lua;
use tracing::{debug, error, info, warn};

pub struct LuaLoader {}

impl LuaLoader {
    pub fn new() -> LuaLoader {
        LuaLoader {}
    }

    pub fn run_scan(&self, script_code: &str, target_url: (&str, &str)) {
        let lua_code = Lua::new();
        let mut all_payloads = Vec::new();
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

            // Set Globals
            global.set("log_info", log_info).unwrap();
            global.set("log_error", log_error).unwrap();
            global.set("log_debug", log_debug).unwrap();
            global.set("log_warn", log_warn).unwrap();
            global.set("change_urlquery", change_url).unwrap();
            global.set("set_urlvalue", set_urlvalue.unwrap()).unwrap();

            // Execute The Script
            lua_context.load(script_code).exec().unwrap();
            let main_func: rlua::Function = global.get("generate_payload").unwrap();
            let urls_payloads = main_func
                .call::<_, rlua::Table>((target_url.0, target_url.1))
                .unwrap();
            urls_payloads.pairs::<i32, String>().for_each(|new_url| {
                all_payloads.push(new_url.unwrap().1);
            });
        });
        self.scan(script_code,all_payloads);
    }

    pub fn scan(&self, script_code: &str, urls: Vec<String>) {
        let threader = rayon::ThreadPoolBuilder::new()
            .num_threads(20)
            .build()
            .unwrap();
        let sender = utils::Sender::init();
        threader.install(|| {
            urls.par_iter().for_each(|current_url| {
                let sender = &sender;
                let resp = sender.send(current_url.to_string());
                let lua_code = Lua::new();
                lua_code.context(|lua_context| {
                    let global = lua_context.globals();
                    // Set Functions
                    let is_match = lua_context
                        .create_function(|_, (pattern, resp): (String, String)| {
                            Ok(utils::is_match(pattern, resp))
                        })
                        .unwrap();
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

                    // Set Globals
                    global.set("log_info", log_info).unwrap();
                    global.set("log_error", log_error).unwrap();
                    global.set("log_debug", log_debug).unwrap();
                    global.set("log_warn", log_warn).unwrap();
                    global.set("is_match", is_match).unwrap();
                    lua_context.load(script_code).exec().unwrap();
                    let main_func: rlua::Function = global.get("scan").unwrap();
                    let out_data = main_func.call::<_, rlua::Table>(resp).unwrap();
                });
            });
        });
    }
}
