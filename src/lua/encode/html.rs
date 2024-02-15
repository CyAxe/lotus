use mlua::prelude::LuaResult;
use mlua::Lua;

pub fn htmlencode<'lua>(_: &'lua Lua,txt: String) -> LuaResult<String>{
    Ok(html_escape::encode_text(&txt).to_string())
}

pub fn htmldecode<'lua>(_: &'lua Lua,txt: String) -> LuaResult<String>{
    Ok(html_escape::decode_html_entities(&txt).to_string())
}
