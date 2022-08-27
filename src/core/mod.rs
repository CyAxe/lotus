mod utils;
use rlua::Lua;

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
            let sender_func = lua_context.create_function(move |_, url: String| {
                Ok(
                    sender.send(url)
                   )
            });
            let regex_func = lua_context.create_function(|_, data: (String,String)| {
                Ok(utils::regex(data))
            });
            global.set("send_req", sender_func.unwrap()).unwrap();
            global.set("is_match", regex_func.unwrap()).unwrap();
            lua_context.load(LUA_CODE).exec().unwrap();
            let main_func: rlua::Function = global.get("main").unwrap();
            let _out = main_func.call::<_, rlua::Table>(url).unwrap();
            println!("FF {:?}", _out.get::<_,bool>("found").unwrap());
        });
    }
}
