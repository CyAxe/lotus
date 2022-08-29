mod utils;
use rlua::Lua;
use log::{info, debug,error, warn};


const LUA_CODE: &str = include_str!("script.lua");


pub struct LuaLoader {}

impl LuaLoader {
    pub fn new() -> LuaLoader {
        LuaLoader {}
    }

    pub fn load_auth(&self, url: String) {
        let lua_code = Lua::new();
        let sender = utils::Sender::init();
        lua_code.context(|lua_context| {
            let global = lua_context.globals();
            // Set Functions
            let sender_func = lua_context.create_function(move |_, url: String| {
                Ok(
                    sender.send(url)
                   )
            });
            let html_search_func = lua_context.create_function(|_, html_data: (String,String)|{
                Ok(utils::html_search(&html_data.0,&html_data.1))
            });
            let html_parse_func = lua_context.create_function(|_, html_data: (String,String)|{
                Ok(utils::html_parse(&html_data.0,&html_data.1))
            }).unwrap();
            let css_selector = lua_context.create_function(|_, html_data: String| {
                Ok(utils::css_selector(&html_data))
            }).unwrap();
            let regex_func = lua_context.create_function(|_, data: (String,String)| {
                Ok(utils::regex(data))
            });
            let log_info = lua_context.create_function(|_, log_msg: String|{
                info!("{}",log_msg);
                Ok(())
            }).unwrap();
            let log_warn = lua_context.create_function(|_, log_msg: String|{
                warn!("{}",log_msg);
                Ok(())
            }).unwrap();
            let log_debug = lua_context.create_function(|_, log_msg: String|{
                debug!("{}",log_msg);
                Ok(())
            }).unwrap();
            let log_error = lua_context.create_function(|_, log_msg: String|{
                error!("{}",log_msg);
                Ok(())
            }).unwrap();
            let change_url = lua_context.create_function(|_, url: (String,String)|{
                Ok(utils::change_urlquery(url.0,url.1))
            }).unwrap();


            // Set Globals
            global.set("send_req", sender_func.unwrap()).unwrap();
            global.set("is_match", regex_func.unwrap()).unwrap();
            global.set("html_search",html_search_func.unwrap()).unwrap();
            global.set("html_parse",html_parse_func).unwrap();
            global.set("css_generate",css_selector).unwrap();
            global.set("log_info",log_info).unwrap();
            global.set("log_error",log_error).unwrap();
            global.set("log_debug",log_debug).unwrap();
            global.set("log_warn",log_warn).unwrap();
            global.set("change_urlquery",change_url).unwrap();


            // Execute The Script
            lua_context.load(LUA_CODE).exec().unwrap();
            let main_func: rlua::Function = global.get("main").unwrap();
            let _out = main_func.call::<_, rlua::Table>(url).unwrap();
            println!("FF {:?} || ", _out.get::<_,bool>("found").unwrap() );
        });
    }
}
