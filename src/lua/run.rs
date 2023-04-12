use crate::lua::runtime::{
    encode_ext::EncodeEXT, http_ext::HTTPEXT, payloads_ext::PayloadsEXT, utils_ext::UtilsEXT,
};
use crate::{
    cli::bar::BAR,
    lua::{
        model::{LuaOptions, LuaRunTime},
        network::http::Sender,
        output::report::AllReports,
    },
    RequestOpts, ScanTypes,
};
use mlua::{Lua, LuaSerdeExt};
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;

#[derive(Clone)]
pub struct LuaLoader {
    output_dir: Option<PathBuf>,
    request: RequestOpts,
}

/// Start Lotus by adding the ProgressBar and http request options
/// * `request` - Request Options
/// * `output_dir` - output file
impl LuaLoader {
    pub fn new(request: RequestOpts, output_dir: Option<PathBuf>) -> LuaLoader {
        Self {
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
            .open(&self.output_dir.as_ref().unwrap())
            .expect("Could not open file")
            .write_all(format!("{}\n", results).as_str().as_bytes())
            .expect("Could not write to file");
    }

    /// Run The Targeted Script on the target url
    /// * `target_url` - Target url
    /// * `target_type` - the input type if its HOST or URL
    pub async fn run_scan<'a>(&self, lua_opts: LuaOptions<'_>) -> Result<(), mlua::Error> {
        let lua = Lua::new_with(mlua::StdLib::ALL_SAFE, mlua::LuaOptions::new()).unwrap();
        let env_vars: mlua::Value = lua.to_value(&lua_opts.env_vars).unwrap();

        lua.globals().set("ENV", env_vars).unwrap();
        lua.globals()
            .set("SCRIPT_PATH", lua_opts.script_dir)
            .unwrap();
        lua.globals()
            .set("FUZZ_WORKERS", lua_opts.fuzz_workers)
            .unwrap();

        match lua_opts.target_type {
            ScanTypes::HOSTS => {
                // for HOSTS, set TARGET_HOST global
                self.set_lua(None, &lua);
                lua.globals()
                    .set(
                        "INPUT_DATA",
                        lua_opts.target_url.unwrap().as_str().unwrap().to_string(),
                    )
                    .unwrap();
            }
            ScanTypes::URLS => {
                // for all other target types, set target URL
                self.set_lua(
                    Some(&&lua_opts.target_url.unwrap().as_str().unwrap().to_string()),
                    &lua,
                );
            }
            ScanTypes::PATHS => {
                self.set_lua(
                    Some(lua_opts.target_url.unwrap().to_string().as_str()),
                    &lua,
                );
            }
            ScanTypes::CUSTOM => {
                let serde_value = serde_json::to_value(lua_opts.target_url).unwrap();
                lua.globals()
                    .set("INPUT_DATA", lua.to_value(&serde_value).unwrap())
                    .unwrap();
                self.set_lua(None, &lua);
            }
        };

        // execute script code
        log::debug!("Executing Code: {}", lua_opts.script_dir);
        let run_code = lua.load(lua_opts.script_code).exec_async().await;
        if let Err(e) = run_code {
            let bar = BAR.lock().unwrap();
            let error_msg = format!("An error occurred while running the script:\n\n{}\n\nPlease check the script code and try again.", e);
            bar.inc(1);
            bar.println(&error_msg);
            return Err(e);
        }

        // call main function
        let main_func = lua.globals().get::<_, mlua::Function>("main");
        if let Err(_) = main_func {
            let msg = format!("The script in directory [{}] does not contain a main function.\n\nThe main function is required to execute the script. Please make sure that the script contains a main function and try again.", lua_opts.script_dir);
            log::error!("{}", msg);
            BAR.lock().unwrap().println(msg);
            return Ok(());
        }

        log::debug!("Calling the main function: {}", lua_opts.script_dir);
        let run_scan = main_func
            .unwrap()
            .call_async::<_, mlua::Value>(mlua::Value::Nil)
            .await;
        BAR.lock().unwrap().inc(1);

        if let Err(e) = run_scan {
            let msg = format!("[{}] Script Error: {:?}", lua_opts.script_dir, e);
            log::error!("{}", msg);
            BAR.lock().unwrap().println(msg);
        } else {
            // process script report
            let script_report = lua.globals().get::<_, AllReports>("Reports").unwrap();
            if !script_report.reports.is_empty() {
                let results = serde_json::to_string(&script_report.reports).unwrap();
                let report_count = script_report.reports.len();
                let log_message = format!(
                    "The script in directory [{}] generated {} report(s):\n\n{}\n\n",
                    lua_opts.script_dir, report_count, results
                );
                log::debug!("{}", log_message);
                if self.output_dir.is_some() {
                    self.write_report(&results);
                }
            }
        }

        Ok(())
    }
}
