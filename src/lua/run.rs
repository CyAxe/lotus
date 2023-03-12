use crate::lua::runtime::{
    encode_ext::EncodeEXT, http_ext::HTTPEXT, payloads_ext::PayloadsEXT, utils_ext::UtilsEXT,
};
use crate::{
    cli::bar::BAR,
    lua::{loader::{LuaRunTime, LuaOptions}, network::http::Sender, output::vuln::AllReports},
    RequestOpts, ScanTypes
};
use mlua::Lua;
use std::{fs::OpenOptions, io::Write};

#[derive(Clone)]
pub struct LuaLoader {
    output_dir: String,
    request: RequestOpts,
}

/// Start Lotus by adding the ProgressBar and http request options
/// * `request` - Request Options
/// * `output_dir` - output file
impl LuaLoader {
    pub fn new(request: RequestOpts, output_dir: String) -> LuaLoader {
        LuaLoader {
            output_dir,
            request,
        }
    }

    /// Set Lua Functions for http and matching
    ///
    fn set_lua(&self, target_url: Option<&str>, lua: &Lua) {
        // Adding Lotus Lua Function
        let lua_eng = LuaRunTime { lua };
        lua_eng.add_httpfuncs(target_url);
        lua_eng.add_encode_function();
        lua_eng.add_printfunc();
        lua_eng.add_matchingfunc();
        lua_eng.add_threadsfunc();
        lua_eng.add_payloadsfuncs();
        lua.globals()
            .set(
                "ERR_STRING",
                lua.create_function(|_, error: mlua::Error| Ok(error.to_string()))
                    .unwrap(),
            )
            .unwrap();
        // HTTP Sender
        lua.globals()
            .set(
                "http",
                Sender::init(
                    self.request.headers.clone(),
                    self.request.proxy.clone(),
                    self.request.timeout,
                    self.request.redirects,
                ),
            )
            .unwrap();
    }
    fn write_report(&self, results: &str) {
        OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open(&self.output_dir)
            .expect("Could not open file")
            .write_all(format!("{}\n", results).as_str().as_bytes())
            .expect("Could not write to file");
    }

    /// Run The Targeted Script on the target url
    /// * `target_url` - Target url
    /// * `target_type` - the input type if its HOST or URL
    pub async fn run_scan<'a>(
        &self,
        lua_opts:  LuaOptions<'_>
    ) -> Result<(), mlua::Error> {
        let lua = Lua::new();
        // settings lua api
        if let ScanTypes::HOSTS = lua_opts.target_type {
            self.set_lua(None, &lua);
            lua.globals()
                .set("TARGET_HOST", lua_opts.target_url.unwrap())
                .unwrap();
        } else {
            self.set_lua(lua_opts.target_url, &lua);
        }
        lua.globals().set("SCRIPT_PATH", lua_opts.script_dir).unwrap();
        lua.globals().set("FUZZ_WORKERS", lua_opts.fuzz_workers).unwrap();

        // Handle this error please
        let run_code = lua.load(lua_opts.script_code).exec_async().await;
        if run_code.is_err() {
            {
                let bar = BAR.lock().unwrap();
                bar.inc(1);
                bar.println("Script Error")
            };
            return run_code;
        }
        let main_func = lua.globals().get::<_, mlua::Function>("main");
        if main_func.is_err() {
            log::error!("[{}] there is no main function, Skipping ..", lua_opts.script_dir);
            {
                BAR.lock().unwrap().println(format!(
                    "[{}] there is no main function, Skipping ..",
                    lua_opts.script_dir
                ))
            };
        } else {
            let run_scan = main_func
                .unwrap()
                .call_async::<_, mlua::Value>(mlua::Value::Nil)
                .await;
            {
                BAR.lock().unwrap().inc(1)
            };
            if run_scan.is_err() {
                log::error!(
                    "[{}] Script Error : {:?}",
                    lua_opts.script_dir,
                    run_scan.clone().unwrap_err()
                );
                {
                    BAR.lock()
                        .unwrap()
                        .println(format!("Script ERROR: {:?}", run_scan.unwrap_err()))
                };
            } else {
                let script_report = lua.globals().get::<_, AllReports>("Reports").unwrap();
                if !script_report.reports.is_empty() {
                    let results = serde_json::to_string(&script_report.reports).unwrap();
                    log::debug!(
                        "[{}] Report Length {}",
                        lua_opts.script_dir,
                        script_report.reports.len()
                    );
                    self.write_report(&results);
                } else {
                    log::debug!("[{}] Script report is empty", lua_opts.script_dir);
                }
            }
        }
        Ok(())
    }
}
