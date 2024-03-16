use crate::{
    lua::{
        model::LuaRunTime,
        parsing::text::ResponseMatcher,
        threads::{LuaThreader, ParamScan},
    },
    BAR,
};
use std::path::Path;
use std::sync::Arc;
use tokio::sync::Mutex;

macro_rules! set_global_function {
    ($lua:expr, $name:expr, $func:expr) => {
        $lua.globals().set($name, $func).unwrap();
    };
}


pub trait UtilsEXT {
    fn add_threadsfunc(&self);
    fn add_matchingfunc(&self);
    fn add_printfunc(&self);
}

impl UtilsEXT for LuaRunTime<'_> {
    fn add_printfunc(&self) {
        set_global_function!(
            self.lua,
            "join_script_dir",
            self.lua
                .create_function(|c_lua, new_path: String| {
                    let script_path = c_lua.globals().get::<_, String>("SCRIPT_PATH").unwrap();
                    let the_path = Path::new(&script_path);
                    Ok(the_path.parent().unwrap().join(new_path).to_str().unwrap().to_string())
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

        set_global_function!(
            self.lua,
            "println",
            self.lua
                .create_function(move |_, msg: String| {
                    BAR.lock().unwrap().println(msg);
                    Ok(())
                })
                .unwrap()
        );
    }

    fn add_matchingfunc(&self) {
        set_global_function!(
            self.lua,
            "Matcher",
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
                set_global_function!(
                    self.lua,
                    $name,
                    self.lua.create_function(|_, (str_one, str_two): (String, String)| {
                        Ok(str_one.$method(&str_two))
                    }).unwrap()
                );
            }};
        }

        string_function!("str_startswith", starts_with);
        string_function!("str_contains", contains);
    }

    fn add_threadsfunc(&self) {
        // ProgressBar
        set_global_function!(
            self.lua,
            "ParamScan",
            ParamScan {
                finds: Arc::new(Mutex::new(false)),
                accept_nil: Arc::new(Mutex::new(false)),
            }
        );

        set_global_function!(
            self.lua,
            "LuaThreader",
            LuaThreader {
                stop: Arc::new(Mutex::new(false)),
            }
        );
    }
}
