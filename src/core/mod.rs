pub mod utils;
use rlua::Lua;
use tracing::{debug, error, info, warn};

pub struct LuaLoader {}

impl LuaLoader {
    pub fn new() -> LuaLoader {
        LuaLoader {}
    }

    pub fn run_scan(&self, script_code: &str, target_url: (&str, &str)) {
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
            let send_req_func = lua_context.create_function(move |_, url: String|{
                Ok(sender.send(url))
            }).unwrap();

            // Set Globals
            global.set("log_info", log_info).unwrap();
            global.set("log_error", log_error).unwrap();
            global.set("log_debug", log_debug).unwrap();
            global.set("log_warn", log_warn).unwrap();
            global.set("change_urlquery", change_url).unwrap();
            global.set("set_urlvalue", set_urlvalue.unwrap()).unwrap();
            global.set("send_req", send_req_func).unwrap();
            global.set("is_match", lua_context.create_function(|_, (pattern, body) : (String,String)|{
                Ok(utils::is_match(pattern,body))
            }).unwrap()).unwrap();

            // Execute The Script
            lua_context.load(script_code).exec().unwrap();
            let main_func: rlua::Function = global.get("main").unwrap();
            let out = main_func
                .call::<_, rlua::Table>(target_url)
                .unwrap();
            out.pairs::<String,String>().for_each(|_d|{
                info!("OUT {:?}",_d);
            });
        });
    }

}
