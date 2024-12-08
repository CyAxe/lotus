use crate::lua::{
    model::LuaRunTime,
    parsing::text::ResponseMatcher,
    threads::{LuaThreader, ParamScan},
};
use crate::utils::bar::GLOBAL_PROGRESS_BAR;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::Mutex;
use mlua::{ExternalError, Lua, Result, Table};

macro_rules! set_nested_function {
    ($table:expr, $name:expr, $func:expr) => {
        $table.set($name, $func).unwrap();
    };
}

fn create_lotus_encode_table(lua: &Lua) -> Result<Table> {
    let globals = lua.globals();

    let lotus = match globals.get::<_, Table>("lotus") {
        Ok(table) => table,
        Err(_) => {
            let table = lua.create_table()?;
            globals.set("lotus", table.clone())?;
            table
        }
    };

    let encode_table = match lotus.get::<_, Table>("encode") {
        Ok(table) => table,
        Err(_) => {
            let table = lua.create_table()?;
            lotus.set("encode", table.clone())?;
            table
        }
    };

    Ok(encode_table)
}

pub trait EncodeEXT {
    fn add_encode_funcs(&self) -> Result<()>;
}

impl EncodeEXT for LuaRunTime<'_> {
    fn add_encode_funcs(&self) -> Result<()> {
        let encode_table = create_lotus_encode_table(&self.lua)?;

        let base64_encode_fn = self.lua.create_function(|_, input: String| {
            Ok(base64::encode(input))
        })?;
        set_nested_function!(encode_table, "base64encode", base64_encode_fn);

        let base64_decode_fn = self.lua.create_function(|_, input: String| {
            match base64::decode(input) {
                Ok(decoded) => Ok(String::from_utf8_lossy(&decoded).to_string()),
                Err(_) => Err("Invalid Base64 input".to_lua_err()),
            }
        })?;
        set_nested_function!(encode_table, "base64decode", base64_decode_fn);

        let url_encode_fn = self.lua.create_function(|_, input: String| {
            Ok(urlencoding::encode(&input).to_string())
        })?;
        set_nested_function!(encode_table, "urlencode", url_encode_fn);

        let url_decode_fn = self.lua.create_function(|_, input: String| {
            match urlencoding::decode(&input) {
                Ok(decoded) => Ok(decoded.to_string()),
                Err(_) => Err("Invalid URL input".to_lua_err()),
            }
        })?;
        set_nested_function!(encode_table, "urldecode", url_decode_fn);

        Ok(())
    }
}

pub trait UtilsEXT {
    fn add_threadsfunc(&self);
    fn add_matchingfunc(&self);
    fn add_printfunc(&self);
}

impl UtilsEXT for LuaRunTime<'_> {
    fn add_printfunc(&self) {
        set_nested_function!(
            self.lua.globals(),
            "join_script_dir",
            self.lua
                .create_function(|c_lua, new_path: String| {
                    let script_path = c_lua.globals().get::<_, String>("SCRIPT_PATH").unwrap();
                    let the_path = Path::new(&script_path);
                    Ok(the_path
                        .parent()
                        .unwrap()
                        .join(new_path)
                        .to_str()
                        .unwrap()
                        .to_string())
                })
                .unwrap()
        );

        macro_rules! log_function {
            ($name:expr, $level:ident) => {{
                let log_func = self
                    .lua
                    .create_function(move |_, log_msg: String| {
                        log::$level!("{}", log_msg);
                        Ok(())
                    })
                    .unwrap();
                self.lua.globals().set($name, log_func).unwrap();
            }};
        }

        log_function!("log_info", info);
        log_function!("log_warn", warn);
        log_function!("log_debug", debug);
        log_function!("log_error", error);

        set_nested_function!(
            self.lua.globals(),
            "println",
            self.lua
                .create_function(move |_, msg: String| {
                    GLOBAL_PROGRESS_BAR
                        .lock()
                        .unwrap()
                        .clone()
                        .unwrap()
                        .println(msg);
                    Ok(())
                })
                .unwrap()
        );
    }

    fn add_matchingfunc(&self) {
        set_nested_function!(
            self.lua.globals(),
            "matcher",
            ResponseMatcher {
                ignore_whitespace: false,
                case_insensitive: false,
                multi_line: false,
                octal: true,
                unicode: true,
                dot_matches_new_line: false,
            }
        );

        macro_rules! string_function {
            ($name:expr, $method:ident) => {{
                set_nested_function!(
                    self.lua.globals(),
                    $name,
                    self.lua
                        .create_function(|_, (str_one, str_two): (String, String)| {
                            Ok(str_one.$method(&str_two))
                        })
                        .unwrap()
                );
            }};
        }

        string_function!("str_startswith", starts_with);
        string_function!("str_contains", contains);
    }

    fn add_threadsfunc(&self) {
        // ProgressBar
        set_nested_function!(
            self.lua.globals(),
            "ParamScan",
            ParamScan {
                finds: Arc::new(Mutex::new(false)),
                accept_nil: Arc::new(Mutex::new(false)),
            }
        );

        set_nested_function!(
            self.lua.globals(),
            "LuaThreader",
            LuaThreader {
                stop: Arc::new(Mutex::new(false)),
            }
        );
    }
}
