use mlua::Lua;
use serde_json::Value as JsonValue;

pub fn custom_input_lua(input_data: Vec<String>, code: &str) -> Result<Vec<JsonValue>, mlua::Error> {
    let lua_env = Lua::new();
    log::debug!("Loading Input handler lua code");
    lua_env.load(code).exec()?;
    let parse_input = lua_env.globals().get::<_, mlua::Function>("parse_input")?;
    log::debug!("Calling parse_input function");
    let output_data = parse_input.call::<_, Vec<mlua::Table>>(input_data)?;
    let final_output: Vec<JsonValue> = output_data
        .into_iter()
        .map(|table| serde_json::to_value(&table).unwrap())
        .collect();
    log::debug!("parse_input function returned {} item",final_output.len());
    Ok(final_output)
}
