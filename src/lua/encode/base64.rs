use mlua::prelude::LuaResult;
use mlua::ExternalError;
use mlua::Lua;

pub fn base64_encode<'lua>(_: &'lua Lua, txt: String) -> LuaResult<String> {
    Ok(base64::encode(txt))
}

pub fn base64_decode<'lua>(_: &'lua Lua, txt: String) -> Result<Vec<u8>, mlua::Error> {
    match base64::decode(txt) {
        Ok(decoded_content) => Ok(decoded_content),
        Err(err) => Err(err.to_lua_err()),
    }
}
