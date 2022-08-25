mod utils;
use rlua::Lua;

const LUA_CODE: &str = include_str!("script.lua");

pub(crate) struct LuaLoader {}

impl LuaLoader {
    pub(crate) fn new() -> LuaLoader {
        LuaLoader {}
    }

    pub(crate) fn load_auth(&self,url :String ) {
        let lua_code = Lua::new();
        let sender = utils::Sender::init();
        
        lua_code.context(|lua_context| {
            let global = lua_context.globals();
            let sender_func =
                lua_context.create_function(move |_,url: String| {
                    Ok(sender.send(url).unwrap())
                });
            global.set("send_req", sender_func.unwrap()).unwrap();
            lua_context.load(LUA_CODE).exec().unwrap();
            let print: rlua::Function = global.get("main").unwrap();
            let _out = print.call::<_, String>(url).unwrap();
        });
    }
}
