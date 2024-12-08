use crate::lua::{
    model::LuaRunTime,
    parsing::text::ResponseMatcher,
    threads::{LuaThreader, ParamScan},
};
use crate::utils::bar::GLOBAL_PROGRESS_BAR;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::Mutex;
use mlua::{Lua, Result, Table};

macro_rules! set_nested_function {
    ($table:expr, $name:expr, $func:expr) => {
        $table.set($name, $func).unwrap();
    };
}

fn create_lotus_table(lua: &Lua) -> Result<Table> {
    let globals = lua.globals();

    let lotus = match globals.get::<_, Table>("lotus") {
        Ok(table) => table,
        Err(_) => {
            let table = lua.create_table()?;
            globals.set("lotus", table.clone())?;
            table
        }
    };

    Ok(lotus)
}

pub trait UtilsEXT {
    fn add_threadsfunc(&self);
    fn add_matchingfunc(&self);
    fn add_printfunc(&self);
}

impl UtilsEXT for LuaRunTime<'_> {
    /// Adds utility functions for printing and logging
    fn add_printfunc(&self) {
        let lotus_table = create_lotus_table(&self.lua).unwrap();

        // Function to join script directory paths
        set_nested_function!(
            lotus_table,
            "join_script_dir",
            self.lua
                .create_function(|lua, new_path: String| {
                    let script_path = lua.globals().get::<_, String>("SCRIPT_PATH")?;
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

        // Macro to simplify log level function creation
        macro_rules! log_function {
            ($name:expr, $level:ident) => {{
                let log_func = self
                    .lua
                    .create_function(move |_, log_msg: String| {
                        log::$level!("{}", log_msg);
                        Ok(())
                    })
                    .unwrap();
                lotus_table.set($name, log_func).unwrap();
            }};
        }

        // Add log functions for various levels
        log_function!("log_info", info);
        log_function!("log_warn", warn);
        log_function!("log_debug", debug);
        log_function!("log_error", error);

        // Add a print function that uses the global progress bar
        set_nested_function!(
            lotus_table,
            "println",
            self.lua
                .create_function(|_, msg: String| {
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

    /// Adds utility functions for string and response matching
    fn add_matchingfunc(&self) {
        let lotus_table = create_lotus_table(&self.lua).unwrap();

        // Add a matcher utility
        set_nested_function!(
            lotus_table,
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

        // Macro to simplify string utility function creation
        macro_rules! string_function {
            ($name:expr, $method:ident) => {{
                set_nested_function!(
                    lotus_table,
                    $name,
                    self.lua
                        .create_function(|_, (str_one, str_two): (String, String)| {
                            Ok(str_one.$method(&str_two))
                        })
                        .unwrap()
                );
            }};
        }

        // Add string utility functions
        string_function!("str_startswith", starts_with);
        string_function!("str_contains", contains);
        string_function!("str_endswith", ends_with);
    }

    /// Adds threading and concurrency-related functions
    fn add_threadsfunc(&self) {
        let lotus_table = create_lotus_table(&self.lua).unwrap();

        // Add ParamScan utility for handling thread parameters
        set_nested_function!(
            lotus_table,
            "ParamScan",
            ParamScan {
                finds: Arc::new(Mutex::new(false)),
                accept_nil: Arc::new(Mutex::new(false)),
            }
        );

        // Add LuaThreader utility for managing threads
        set_nested_function!(
            lotus_table,
            "LuaThreader",
            LuaThreader {
                stop: Arc::new(Mutex::new(false)),
            }
        );

        // Add a function for thread-safe logging
        set_nested_function!(
            lotus_table,
            "thread_log",
            self.lua
                .create_function(|_, log_msg: String| {
                    log::info!("Thread log: {}", log_msg);
                    Ok(())
                })
                .unwrap()
        );
    }
}
