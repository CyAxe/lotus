use crate::{
    lua::{
        output::report::AllReports,
        parsing::{files::filename_to_string, url::HttpMessage},
    },
    CliErrors, LuaRunTime,
};
use log::{debug, error, info, warn};
use mlua::ExternalError;
use std::collections::HashMap;
use std::path::Path;
use std::time::Duration;
use tokio::time::sleep;
use url::Url;

pub trait HTTPEXT {
    fn add_httpfuncs(&self, target_url: Option<&str>);
}

impl HTTPEXT for LuaRunTime<'_> {
    fn add_httpfuncs(&self, target_url: Option<&str>) {
        self.lua
            .globals()
            .set(
                "JOIN_SCRIPT_DIR",
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
        let headers_converter = self
            .lua
            .create_function(|_, headers_txt: String| {
                let mut result = HashMap::new();
                for line in headers_txt.lines() {
                    if let Some((name, value)) = line.split_once(":") {
                        result.insert(name.trim().to_string(), value.trim().to_string());
                    }
                }
                Ok(result)
            })
            .unwrap();
        self.lua
            .globals()
            .set("make_headers", headers_converter)
            .unwrap();
        self.lua
            .globals()
            .set(
                "sleep",
                self.lua
                    .create_async_function(|_, time: u64| async move {
                        sleep(Duration::from_secs(time)).await;
                        Ok(())
                    })
                    .unwrap(),
            )
            .unwrap();

        self.lua
            .globals()
            .set(
                "pathjoin",
                self.lua
                    .create_function(|_, (current_path, new_path): (String, String)| {
                        let the_path = std::path::Path::new(&current_path).join(new_path);
                        Ok(the_path.to_str().unwrap().to_string())
                    })
                    .unwrap(),
            )
            .unwrap();
        self.lua
            .globals()
            .set(
                "readfile",
                self.lua
                    .create_function(|_ctx, file_path: String| {
                        if Path::new(&file_path).exists() {
                            let file_content = filename_to_string(&file_path)?;
                            Ok(file_content)
                        } else {
                            Err(CliErrors::ReadingError.to_lua_err())
                        }
                    })
                    .unwrap(),
            )
            .unwrap();
        self.lua.globals().set("log_info", log_info).unwrap();
        self.lua.globals().set("log_error", log_error).unwrap();
        self.lua.globals().set("log_debug", log_debug).unwrap();
        self.lua.globals().set("log_warn", log_warn).unwrap();

        if !target_url.is_none() {
            self.lua
                .globals()
                .set(
                    "HttpMessage",
                    HttpMessage {
                        url: Url::parse(target_url.unwrap()).unwrap(),
                    },
                )
                .unwrap();
        }
        self.lua
            .globals()
            .set(
                "Reports",
                AllReports {
                    reports: Vec::new(),
                },
            )
            .unwrap();
    }
}
