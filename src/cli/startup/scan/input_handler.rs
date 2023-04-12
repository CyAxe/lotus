use crate::lua::model::LuaRunTime;
use crate::lua::runtime::http_ext::HTTPEXT;
use crate::lua::runtime::utils_ext::UtilsEXT;
use mlua::{Lua, StdLib};
use serde_json::Value as JsonValue;

pub fn custom_input_lua(
    input_data: Vec<String>,
    code: &str,
) -> Result<Vec<JsonValue>, mlua::Error> {
    let lua_env = LuaRunTime {
        lua: &unsafe { Lua::unsafe_new_with(StdLib::ALL_SAFE, mlua::LuaOptions::new()) },
    };
    lua_env.add_printfunc();
    lua_env.add_matchingfunc();
    lua_env.add_httpfuncs(None);
    log::debug!("Loading Input handler lua code");
    lua_env.lua.load(code).exec()?;
    let parse_input = lua_env
        .lua
        .globals()
        .get::<_, mlua::Function>("parse_input")?;
    log::debug!("Calling parse_input function");
    let output_data = parse_input.call::<_, Vec<mlua::Value>>(input_data)?;
    let final_output: Vec<JsonValue> = output_data
        .into_iter()
        .map(|table| serde_json::to_value(&table).unwrap())
        .collect();
    log::debug!("parse_input function returned {} item", final_output.len());
    Ok(final_output)
}
