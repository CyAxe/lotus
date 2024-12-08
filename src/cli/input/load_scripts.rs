use crate::cli::input::parse_requests::FullRequest;
use crate::lua::{
    model::LuaRunTime,
    runtime::{encode_ext::EncodeEXT, http_ext::HTTPEXT, utils_ext::UtilsEXT},
};
use crate::{filename_to_string, CliErrors};
use glob::glob;
use log::error;
use mlua::Lua;
use std::path::PathBuf;

/// Return Vector of scripts name and code with both methods
pub fn get_scripts(script_path: PathBuf) -> Vec<(String, String)> {
    let paths: Vec<&str> = script_path.to_str().unwrap().split(',').collect();
    let mut scripts = vec![];

    for path in paths {
        let mut file_path = PathBuf::from(path.trim());
        if file_path.is_dir() {
            file_path.push("*.lua");
        }
        let loaded_scripts = load_scripts(&file_path);
        if loaded_scripts.is_err() {
            log::error!(
                "{}",
                &format!("Loading scripts error: {}", loaded_scripts.unwrap_err()),
            );
            std::process::exit(1);
        }
        scripts.extend(loaded_scripts.unwrap());
    }

    scripts
}
/// Use glob patterns to get script path and content based on script path or directory
/// This Function will return a Tuples in Vector with script path and content
fn load_scripts(script_path: &PathBuf) -> Result<Vec<(String, String)>, CliErrors> {
    let mut scripts = Vec::new();
    for entry in
        glob(script_path.as_os_str().to_str().unwrap()).expect("Failed to read glob pattern")
    {
        match entry {
            Ok(path) => scripts.push((
                filename_to_string(path.to_str().unwrap()).unwrap(),
                path.to_str().unwrap().to_string(),
            )),
            Err(e) => error!("{}", e.to_string()),
        }
    }
    Ok(scripts)
}

/// Validating the script code by running the scripts with example input based on the script
/// type `example.com` or `https:///example.com`
/// this function may removing some scripts from the list if it contains errors
/// or it doesn't have a `main` function
/// make sure your lua script contains `SCAN_TYPE` and `main` Function
/// -----
/// * `scripts` - The Scripts Vector contains Vec<(script_path, script_code)>
/// * `number_scantype` - The Scanning type number | 1 = HOST , 2 = URL
pub fn valid_scripts(
    scripts: Vec<(String, String)>,
    scan_type: &str,
) -> Vec<(String, String)> {
    let mut test_target_url: Option<&str> = None;
    let mut test_target_host: Option<&str> = None;
    let mut test_http_msg: Option<FullRequest> = None;

    log::debug!("Checking Scan Type: {}", scan_type);

    match scan_type {
        "FULL_HTTP" => test_http_msg = Some(FullRequest::default()),
        "HOSTS" => test_target_host = Some("example.com"),
        _ => test_target_url = Some("https://example.com"),
    }

    let lua_eng = LuaRunTime {
        lua: unsafe { &Lua::unsafe_new_with(mlua::StdLib::ALL_SAFE, mlua::LuaOptions::new()) },
    };

    let _ = lua_eng.add_encode_function();
    lua_eng.add_printfunc();
    lua_eng.add_matchingfunc();
    lua_eng.add_threadsfunc();

    if let Some(host) = test_target_host {
        let _ = lua_eng.add_httpfuncs(None, None);
        lua_eng.lua.globals().set("INPUT_DATA", host).unwrap();
    } else if let Some(http_msg) = test_http_msg {
        let _ = lua_eng.add_httpfuncs(None, Some(http_msg));
    } else {
        lua_eng.lua.globals().set("INPUT_DATA", Vec::<&str>::new()).unwrap();
        let _ = lua_eng.add_httpfuncs(test_target_url, None);
    }

    let mut used_scripts: Vec<(String, String)> = Vec::new();
    for (script_code, script_path) in scripts {
        lua_eng
            .lua
            .globals()
            .set("SCRIPT_PATH", script_path.clone())
            .unwrap();

        match lua_eng.lua.load(&script_code).exec() {
            Ok(_) => {
                let globals = lua_eng.lua.globals();
                match globals.get::<_, String>("SCAN_TYPE") {
                    Ok(loaded_scan_type) if loaded_scan_type == scan_type => {
                        used_scripts.push((script_code.clone(), script_path.clone()));
                    }
                    _ => {
                        log::error!(
                            "Invalid script type for {}: expected {}, got {:?}",
                            script_path,
                            scan_type,
                            globals.get::<_, String>("SCAN_TYPE")
                        );
                    }
                }
            }
            Err(e) => {
                log::error!("Unable to load script {}: {}", script_path, e);
            }
        }
    }

    log::debug!("Loaded scripts: {:?}", used_scripts);
    used_scripts
}
