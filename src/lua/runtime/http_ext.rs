use crate::{
    cli::input::parse_requests::FullRequest,
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
use url::Url;

macro_rules! set_global_function {
    ($lua:expr, $name:expr, $func:expr) => {
        $lua.globals().set($name, $func).unwrap();
    };
}

macro_rules! create_and_set_global_function {
    ($lua:expr, $name:expr, $func:expr) => {
        set_global_function!($lua, $name, $lua.create_function($func).unwrap());
    };
}

pub trait HTTPEXT {
    fn add_httpfuncs(&self, target_url: Option<&str>, full_httpmsg: Option<FullRequest>);
}

impl HTTPEXT for LuaRunTime<'_> {
    fn add_httpfuncs(&self, target_url: Option<&str>, full_httpmsg: Option<FullRequest>) {
        set_global_function!(
            self.lua,
            "show_response",
            self.lua.create_function(show_response).unwrap()
        );

        if let Some(full_httpmsg) = full_httpmsg {
            self.lua.globals().set("full_req", full_httpmsg).unwrap();
        }

        create_and_set_global_function!(
            self.lua,
            "make_headers",
            |_, headers_txt: String| {
                let mut result = HashMap::new();
                for line in headers_txt.lines() {
                    if let Some((name, value)) = line.split_once(':') {
                        result.insert(name.trim().to_string(), value.trim().to_string());
                    }
                }
                Ok(result)
            }
        );

        create_and_set_global_function!(
            self.lua,
            "pathjoin",
            |_, (current_path, new_path): (String, String)| {
                let the_path = std::path::Path::new(&current_path).join(new_path);
                Ok(the_path.to_str().unwrap().to_string())
            }
        );

        create_and_set_global_function!(
            self.lua,
            "readfile",
            |_ctx, file_path: String| {
                if Path::new(&file_path).exists() {
                    let file_content = filename_to_string(&file_path)?;
                    Ok(file_content)
                } else {
                    Err(CliErrors::ReadingError.to_lua_err())
                }
            }
        );

        let http_message = if let Some(url) = target_url {
            HttpMessage { url: Some(Url::parse(url).unwrap()) }
        } else {
            HttpMessage { url: None }
        };
        set_global_function!(self.lua, "HttpMessage", http_message);

        set_global_function!(
            self.lua,
            "Reports",
            AllReports { reports: Vec::new() }
        );
    }
}
