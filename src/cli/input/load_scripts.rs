use crate::lua::runtime::{
    encode_ext::EncodeEXT, http_ext::HTTPEXT, payloads_ext::PayloadsEXT, utils_ext::UtilsEXT,
};
use crate::{filename_to_string, show_msg, CliErrors, LuaRunTime, MessageLevel};
use glob::glob;
use log::error;
use mlua::Lua;
use std::path::PathBuf;

/// Return Vector of scripts name and code with both methods
pub fn get_scripts(script_path: PathBuf) -> Vec<(String, String)> {
    let loaded_scripts = {
        if script_path.is_dir() {
            load_scripts(script_path)
        } else {
            load_script(script_path)
        }
    };
    if loaded_scripts.is_err() {
        show_msg(
            &format!("Loading scripts error: {}", loaded_scripts.unwrap_err()),
            MessageLevel::Error,
        );
        std::process::exit(1);
    }
    loaded_scripts.unwrap()
}
/// Use glob patterns to get script path and content based on script path or directory
/// This Function will return a Tuples in Vector with script path and content
fn load_scripts(script_path: PathBuf) -> Result<Vec<(String, String)>, CliErrors> {
    let mut scripts = Vec::new();
    for entry in glob(format!("{}{}", script_path.to_str().unwrap(), "/*.lua").as_str())
        .expect("Failed to read glob pattern")
    {
        match entry {
            Ok(path) => scripts.push((
                filename_to_string(path.to_str().unwrap()).unwrap(),
                path.file_name().unwrap().to_str().unwrap().to_string(),
            )),
            Err(e) => error!("{:?}", e),
        }
    }
    return Ok(scripts);
}

/// Loading script based on the script path (without glob)
fn load_script(script_path: PathBuf) -> Result<Vec<(String, String)>, CliErrors> {
    let mut scripts = Vec::new();
    let script_path = script_path.clone();
    let read_script_code = filename_to_string(script_path.to_str().unwrap());
    if read_script_code.is_err() {
        Err(CliErrors::ReadingError)
    } else {
        scripts.push((
            read_script_code.unwrap(),
            script_path.to_str().unwrap().to_string(),
        ));
        return Ok(scripts);
    }
}
/// Validating the script code by running the scripts with example input based on the script
/// type `example.com` or `https:///example.com`
/// this function may removing some scripts from the list if it contains errors
/// or it doesn't have a `main` function
/// make sure your lua script contains `SCAN_TYPE` and `main` Function
/// -----
/// * `bar` - ProgressBar
/// * `scripts` - The Scripts Vector contains Vec<(script_path, script_code)>
/// * `number_scantype` - The Scanning type number | 1 = HOST , 2 = URL
pub fn valid_scripts(
    scripts: Vec<(String, String)>,
    number_scantype: usize,
) -> Vec<(String, String)> {
    let mut test_target_url: Option<&str> = None;
    let mut test_target_host: Option<&str> = None;
    match number_scantype {
        1 => {
            test_target_host = Some("example.com");
        }
        2 => {
            test_target_url = Some("https://example.com");
        }
        _ => {}
    }
    let lua_eng = LuaRunTime { lua: &Lua::new() };
    lua_eng.add_encode_function();
    lua_eng.add_printfunc();
    lua_eng.add_matchingfunc();
    lua_eng.add_threadsfunc();
    lua_eng.add_payloadsfuncs();
    if test_target_host.is_some() {
        lua_eng.add_httpfuncs(None);
        lua_eng
            .lua
            .globals()
            .set("TARGET_HOST", "example.com")
            .unwrap();
    } else {
        lua_eng.add_httpfuncs(test_target_url);
    }
    let mut used_scripts: Vec<(String, String)> = Vec::new();
    scripts.iter().for_each(|(script_code, script_path)| {
        lua_eng
            .lua
            .globals()
            .set("SCRIPT_PATH", script_path.to_string())
            .unwrap();
        let code = lua_eng.lua.load(script_code).exec();
        if code.is_err() {
            show_msg(
                &format!("Unable to load {} script", script_path),
                MessageLevel::Error,
            );
            log::error!(
                "Script Loading Error {} : {}",
                script_path,
                code.unwrap_err()
            );
        } else {
            let global = lua_eng.lua.globals();
            let scan_type = global.get::<_, usize>("SCAN_TYPE".to_string());
            if scan_type.is_err() {
                show_msg(
                    &format!(
                        "Unvalid Script Type {}: {}",
                        script_path,
                        scan_type.unwrap_err().to_string()
                    ),
                    MessageLevel::Error,
                );
            } else {
                if scan_type.unwrap() == number_scantype {
                    used_scripts.push((script_code.into(), script_path.into()));
                }
            }
        }
    });
    used_scripts
}
