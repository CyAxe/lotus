use crate::{
    lua::{
        model::LuaRunTime,
        output::report::AllReports,
        parsing::{files::filename_to_string, req::show_response, url::HttpMessage},
    },
    CliErrors,
};
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
                "show_response",
                self.lua
                    .create_function(show_response)
                    .unwrap(),
            )
            .unwrap();
        let headers_converter = self
            .lua
            .create_function(|_, headers_txt: String| {
                let mut result = HashMap::new();
                for line in headers_txt.lines() {
                    if let Some((name, value)) = line.split_once(':') {
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

        if let Some(url) = target_url {
            self.lua
                .globals()
                .set(
                    "HttpMessage",
                    HttpMessage {
                        url: Some(Url::parse(url).unwrap()),
                    },
                )
                .unwrap();
        } else {
            self.lua
                .globals()
                .set("HttpMessage", HttpMessage { url: None })
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
