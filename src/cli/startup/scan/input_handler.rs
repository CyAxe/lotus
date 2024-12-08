use crate::lua::model::LuaRunTime;
use crate::lua::runtime::http_ext::HTTPEXT;
use crate::lua::runtime::utils_ext::UtilsEXT;
use mlua::{Lua, StdLib};
use serde_json::Value as JsonValue;

/// Executes custom Lua code to process input data and returns the output as JSON values.
///
/// # Arguments
/// * `input_data` - A vector of strings representing the input data to be processed.
/// * `code` - The Lua code to execute, which defines the `parse_input` function.
///
/// # Returns
/// * A vector of JSON values representing the processed output data.
///
/// # Errors
/// * Returns an `mlua::Error` if there are issues with the Lua execution or parsing.
pub fn custom_input_lua(
    input_data: Vec<String>,
    code: &str,
) -> Result<Vec<JsonValue>, mlua::Error> {
    // Initialize a Lua runtime with safe standard libraries.
    let lua_env = LuaRunTime {
        lua: &unsafe { Lua::unsafe_new_with(StdLib::ALL_SAFE, mlua::LuaOptions::new()) },
    };

    // Register custom utility functions for the Lua runtime.
    lua_env.add_printfunc();
    lua_env.add_matchingfunc();
    lua_env.add_httpfuncs(None, None);

    // Load and execute the provided Lua code to define the `parse_input` function.
    log::debug!("Loading Input handler lua code");
    lua_env.lua.load(code).exec()?;

    // Retrieve the `parse_input` function from the Lua environment.
    let parse_input = lua_env
        .lua
        .globals()
        .get::<_, mlua::Function>("parse_input")?;

    // Execute the `parse_input` function with the input data.
    log::debug!("Calling parse_input function");
    let output_data = parse_input.call::<_, Vec<mlua::Value>>(input_data)?;

    // Convert the Lua table output to JSON values.
    let final_output: Vec<JsonValue> = output_data
        .into_iter()
        .map(|table| serde_json::to_value(&table).unwrap())
        .collect();

    // Log the number of items returned by the `parse_input` function.
    log::debug!("parse_input function returned {} item(s)", final_output.len());

    Ok(final_output)
}
