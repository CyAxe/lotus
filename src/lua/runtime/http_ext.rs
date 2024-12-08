use mlua::{ExternalError, Lua, Result, Table};
use url::Url;
use std::collections::HashMap;
use std::path::Path;

use crate::cli::errors::CliErrors;
use crate::cli::input::parse_requests::FullRequest;
use crate::lua::model::LuaRunTime;
use crate::lua::output::report::AllReports;
use crate::lua::parsing::files::filename_to_string;
use crate::lua::parsing::req::show_response;
use crate::lua::parsing::url::HttpMessage;

macro_rules! set_nested_function {
    ($table:expr, $name:expr, $func:expr) => {
        $table.set($name, $func)?;
    };
}

fn create_lotus_http_tables(lua: &Lua) -> Result<Table> {
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

pub trait HTTPEXT {
    fn add_httpfuncs(&self, target_url: Option<&str>, full_httpmsg: Option<FullRequest>) -> Result<()>;
}

impl HTTPEXT for LuaRunTime<'_> {
    fn add_httpfuncs(&self, target_url: Option<&str>, full_httpmsg: Option<FullRequest>) -> Result<()> {
        let lotus_table = create_lotus_http_tables(&self.lua)?;

        // Create and populate an HTTP table
        let http_table = self.lua.create_table()?;

        http_table.set("show_response", self.lua.create_function(show_response)?)?;
        http_table.set("send", self.lua.create_function(|_, opts: mlua::Value| {
            Ok(format!("HTTP request sent with options: {:?}", opts))
        })?)?;

        // Add a `path` function to extract the path from a URL
        http_table.set("path", self.lua.create_function(|_, url: String| {
            let parsed_url = Url::parse(&url).map_err(|e| e.to_lua_err())?;
            Ok(parsed_url.path().to_string())
        })?)?;

        lotus_table.set("http", http_table)?;

        // Add headers function
        let headers_fn = self.lua.create_function(|_, headers_txt: String| {
            let mut result = HashMap::new();
            for line in headers_txt.lines() {
                if let Some((name, value)) = line.split_once(':') {
                    result.insert(name.trim().to_string(), value.trim().to_string());
                }
            }
            Ok(result)
        })?;
        set_nested_function!(lotus_table, "headers", headers_fn);

        // Add paths function
        let paths_fn = self.lua.create_function(|_, (current_path, new_path): (String, String)| {
            let the_path = std::path::Path::new(&current_path).join(new_path);
            Ok(the_path.to_str().unwrap().to_string())
        })?;
        set_nested_function!(lotus_table, "paths", paths_fn);

        // Add file function
        let file_fn = self.lua.create_function(|_, file_path: String| {
            if Path::new(&file_path).exists() {
                let file_content = filename_to_string(&file_path)?;
                Ok(file_content)
            } else {
                Err(CliErrors::ReadingError.to_lua_err())
            }
        })?;
        set_nested_function!(lotus_table, "file", file_fn);

        // Add request data
        if let Some(full_httpmsg) = full_httpmsg {
            let request_table = self.lua.create_table()?;
            request_table.set("message", full_httpmsg)?;
            lotus_table.set("request", request_table)?;
        }

        // Add message function
        let message_fn = if let Some(url) = target_url {
            HttpMessage {
                url: Some(Url::parse(url).unwrap()),
            }
        } else {
            HttpMessage { url: None }
        };
        set_nested_function!(lotus_table, "message", message_fn);

        // Add reports function
        let reports_fn = AllReports {
            reports: Vec::new(),
        };
        set_nested_function!(lotus_table, "reports", reports_fn);

        // Add error handling function
        let error_fn = self.lua.create_function(|_, error: mlua::Error| Ok(error.to_string()))?;
        set_nested_function!(lotus_table, "error", error_fn);

        // Add action function
        let action_fn = self.lua.create_function(|_, input: String| {
            Ok(format!("Action received: {}", input))
        })?;
        set_nested_function!(lotus_table, "action", action_fn);

        Ok(())
    }
}
