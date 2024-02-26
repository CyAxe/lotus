use mlua::prelude::LuaResult;
use mlua::Lua;

pub fn urlencode<'lua>(_: &'lua Lua, txt: String) -> LuaResult<String>{
    Ok(url_escape::encode_fragment(&txt).to_string())
}

pub fn urldecode<'lua>(_: &'lua Lua, txt: String) -> LuaResult<String>{
    Ok(url_escape::decode(&txt).to_string())
}

