use crate::{
    lua::{
        model::LuaRunTime,
        parsing::text::ResponseMatcher,
        threads::{LuaThreader, ParamScan},
    },
    BAR,
};
use std::sync::{Arc, Mutex};
use std::path::Path;
use log::{debug, error, info, warn};


pub trait UtilsEXT {
    fn add_threadsfunc(&self);
    fn add_matchingfunc(&self);
    fn add_printfunc(&self);
}

impl UtilsEXT for LuaRunTime<'_> {
    fn add_printfunc(&self) {
        self.lua
            .globals()
            .set(
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
                    .unwrap(),
            )
            .unwrap();

        let log_info = self
            .lua
            .create_function(|_, log_msg: String| {
                info!("{}", log_msg);
                Ok(())
            })
            .unwrap();
        let log_warn = self
            .lua
            .create_function(|_, log_msg: String| {
                warn!("{}", log_msg);
                Ok(())
            })
            .unwrap();
        let log_debug = self
            .lua
            .create_function(|_, log_msg: String| {
                debug!("{}", log_msg);
                Ok(())
            })
            .unwrap();
        let log_error = self
            .lua
            .create_function(|_, log_msg: String| {
                error!("{}", log_msg);
                Ok(())
            })
            .unwrap();

        self.lua.globals().set("log_info", log_info).unwrap();
        self.lua.globals().set("log_error", log_error).unwrap();
        self.lua.globals().set("log_debug", log_debug).unwrap();
        self.lua.globals().set("log_warn", log_warn).unwrap();
        self.lua
            .globals()
            .set(
                "println",
                self.lua
                    .create_function(move |_, msg: String| {
                        {
                            BAR.lock().unwrap().println(&msg)
                        };
                        Ok(())
                    })
                    .unwrap(),
            )
            .unwrap();
    }
    fn add_matchingfunc(&self) {
        self.lua
            .globals()
            .set(
                "Matcher",
                ResponseMatcher {
                    ignore_whitespace: false,
                    case_insensitive: false,
                    multi_line: false,
                    octal: true,
                    unicode: true,
                    dot_matches_new_line: false,
                },
            )
            .unwrap();

        self.lua
            .globals()
            .set(
                "str_startswith",
                self.lua
                    .create_function(|_, (str_one, str_two): (String, String)| {
                        Ok(str_one.starts_with(&str_two))
                    })
                    .unwrap(),
            )
            .unwrap();
        self.lua
            .globals()
            .set(
                "str_contains",
                self.lua
                    .create_function(|_, (str_one, str_two): (String, String)| {
                        Ok(str_one.contains(&str_two))
                    })
                    .unwrap(),
            )
            .unwrap();
    }

    fn add_threadsfunc(&self) {
        // ProgressBar
        self.lua
            .globals()
            .set(
                "ParamScan",
                ParamScan {
                    finds: Arc::new(Mutex::new(false)),
                    accept_nil: Arc::new(Mutex::new(false)),
                },
            )
            .unwrap();
        self.lua
            .globals()
            .set(
                "LuaThreader",
                LuaThreader {
                    stop: Arc::new(Mutex::new(false)),
                },
            )
            .unwrap();
    }
}
