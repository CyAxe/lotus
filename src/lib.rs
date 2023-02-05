/*
 * This file is part of Lotus Project, an Web Security Scanner written in Rust based on Lua Scripts
 * For details, please see https://github.com/rusty-sec/lotus/
 *
 * Copyright (c) 2022 - Khaled Nassar
 *
 * Please note that this file was originally released under the
 * GNU General Public License as published by the Free Software Foundation;
 * either version 2 of the License, or (at your option) any later version.
 *
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

pub mod cli;
pub mod lua;

use cli::{
    bar::{create_progress, show_msg, MessageLevel},
    errors::CliErrors,
};
use lua::{loader::LuaRunTime, parsing::files::filename_to_string, scan::LuaLoader};
use mlua::Lua;

use futures::{stream, StreamExt};
use reqwest::header::HeaderMap;

use glob::glob;
use log::error;

use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};

/// Lotus HTTP Options
#[derive(Clone)]
pub struct RequestOpts {
    /// Default Headers
    pub headers: HeaderMap,
    /// Custom http Proxy
    pub proxy: Option<String>,
    /// Request Timeout
    pub timeout: u64,
    /// Limits of http redirects
    pub redirects: u32,
}

/// Scanning type For `SCAN_TYPE` in lua scripts
#[derive(Clone, Copy)]
pub enum ScanTypes {
    /// URLS Scanning under ID number 2
    URLS,
    /// HOSTS Scanning under ID number 1
    HOSTS,
}

pub struct Lotus {
    /// Script Path
    pub script_path: PathBuf,
    /// Output Path
    pub output: Option<PathBuf>,
    /// Workers Option
    pub workers: usize,
    /// How many url per script
    pub script_workers: usize,
    /// Stop After X of errors
    pub stop_after: Arc<Mutex<i32>>,
}

impl Lotus {
    /// Return Vector of scripts name and code with both methods
    fn get_scripts(&self) -> Vec<(String, String)> {
        let loaded_scripts = {
            if self.script_path.is_dir() {
                self.load_scripts()
            } else {
                self.load_script()
            }
        };
        if loaded_scripts.is_err() {
            show_msg(
                &format!("Loading scripts error: {}", loaded_scripts.unwrap_err()),
                MessageLevel::Error,
            );
            std::process::exit(1);
        }
        loaded_scripts.unwrap()
    }
    /// Use glob patterns to get script path and content based on script path or directory
    /// This Function will return a Tuples in Vector with script path and content
    fn load_scripts(&self) -> Result<Vec<(String, String)>, CliErrors> {
        let mut scripts = Vec::new();
        for entry in glob(format!("{}{}", self.script_path.to_str().unwrap(), "/*.lua").as_str())
            .expect("Failed to read glob pattern")
        {
            match entry {
                Ok(path) => scripts.push((
                    filename_to_string(path.to_str().unwrap()).unwrap(),
                    path.file_name().unwrap().to_str().unwrap().to_string(),
                )),
                Err(e) => error!("{:?}", e),
            }
        }
        return Ok(scripts);
    }

    /// Loading script based on the script path (without glob)
    fn load_script(&self) -> Result<Vec<(String, String)>, CliErrors> {
        let mut scripts = Vec::new();
        let script_path = &self.script_path.clone();
        let read_script_code = filename_to_string(script_path.to_str().unwrap());
        if read_script_code.is_err() {
            Err(CliErrors::ReadingError)
        } else {
            scripts.push((
                read_script_code.unwrap(),
                script_path.to_str().unwrap().to_string(),
            ));
            return Ok(scripts);
        }
    }
    /// Validating the script code by running the scripts with example input based on the script
    /// type `example.com` or `https:///example.com`
    /// this function may removing some scripts from the list if it contains errors
    /// or it doesn't have a `main` function
    /// make sure your lua script contains `SCAN_TYPE` and `main` Function
    /// -----
    /// * `bar` - ProgressBar
    /// * `scripts` - The Scripts Vector contains Vec<(script_path, script_code)>
    /// * `number_scantype` - The Scanning type number | 1 = HOST , 2 = URL
    fn valid_scripts(
        &self,
        bar: indicatif::ProgressBar,
        scripts: Vec<(String, String)>,
        number_scantype: usize,
    ) -> Vec<(String, String)> {
        let mut test_target_url: Option<&str> = None;
        let mut test_target_host: Option<&str> = None;
        match number_scantype {
            1 => {
                test_target_host = Some("example.com");
            }
            2 => {
                test_target_url = Some("https://example.com");
            }
            _ => {}
        }
        let lua_eng = LuaRunTime {
            lua: &Lua::new(),
            prog: &bar,
        };
        if test_target_host.is_some() {
            lua_eng.setup(None);
            lua_eng
                .lua
                .globals()
                .set("TARGET_HOST", "example.com")
                .unwrap();
        } else {
            lua_eng.setup(test_target_url);
        }
        let mut used_scripts: Vec<(String, String)> = Vec::new();
        scripts.iter().for_each(|(script_code, script_path)| {
            lua_eng
                .lua
                .globals()
                .set("SCRIPT_PATH", script_path.to_string())
                .unwrap();
            let code = lua_eng.lua.load(script_code).exec();
            if code.is_err() {
                show_msg(
                    &format!("Unable to load {} script", script_path),
                    MessageLevel::Error,
                );
                log::error!(
                    "Script Loading Error {} : {}",
                    script_path,
                    code.unwrap_err()
                );
            } else {
                let global = lua_eng.lua.globals();
                let scan_type = global.get::<_, usize>("SCAN_TYPE".to_string());
                if scan_type.is_err() {
                    show_msg(
                        &format!(
                            "Unvalid Script Type {}: {}",
                            script_path,
                            scan_type.unwrap_err().to_string()
                        ),
                        MessageLevel::Error,
                    );
                } else {
                    if scan_type.unwrap() == number_scantype {
                        used_scripts.push((script_code.into(), script_path.into()));
                    }
                }
            }
        });
        used_scripts
    }
    /// Run The Lua Script with real target
    /// * `target_data` - Vector with target urls
    /// * `request_option` - RequestOpts contains some http request options like (proxy , timeout)
    /// * `scan_type` - scan type if its host or url scanning
    /// * `exit_after` - exit after how many of errors
    pub async fn start(
        &self,
        target_data: Vec<String>,
        request_option: RequestOpts,
        scan_type: ScanTypes,
        exit_after: i32,
    ) {
        let bar = create_progress(target_data.len() as u64); // fake bar for testing
        let loaded_scripts = {
            if let ScanTypes::HOSTS = scan_type {
                let scripts = self.get_scripts();
                let loaded_scripts = self.valid_scripts(bar, scripts, 1);
                log::debug!("Running Host scan {:?}", loaded_scripts.len());
                loaded_scripts
            } else {
                let scripts = self.get_scripts();
                let loaded_scripts = self.valid_scripts(bar, scripts, 2);
                log::debug!("Running URL scan {:?}", loaded_scripts.len());
                loaded_scripts
            }
        };
        let bar = create_progress(target_data.len() as u64 * loaded_scripts.len() as u64);
        if self.output.is_none() {
            show_msg("Output argument is missing", MessageLevel::Error);
            std::process::exit(1);
        }
        let lotus_obj = Arc::new(LuaLoader::new(
            &bar,
            request_option.clone(),
            self.output.as_ref().unwrap().to_str().unwrap().to_string(),
        ));
        let scan_type = Arc::new(scan_type);
        stream::iter(target_data)
            .map(move |script_data| {
                let loaded_scripts = loaded_scripts.clone();
                let lotus_loader = Arc::clone(&lotus_obj);
                let scan_type = Arc::clone(&scan_type);
                stream::iter(loaded_scripts.into_iter())
                    .map(move |(script_out, script_name)| {
                        let script_data = script_data.clone();
                        let lotus_loader = Arc::clone(&lotus_loader);
                        let scan_type = Arc::clone(&scan_type);
                        let error_check = {
                            if *self.stop_after.lock().unwrap() == exit_after {
                                log::debug!("Ignoring scripts");
                                false
                            } else {
                                log::debug!("Running {} script on {} ", script_name, script_data);
                                true
                            }
                        };
                        async move {
                            if error_check == false {
                                // Nothing
                            } else {
                                let run_scan = lotus_loader
                                    .run_scan(
                                        Some(script_data.as_str()),
                                        scan_type,
                                        None,
                                        &script_out,
                                        &script_name,
                                    )
                                    .await;
                                if run_scan.is_err() {
                                    log::error!(
                                        "script error: {}",
                                        &run_scan.clone().unwrap_err().to_string()
                                    );
                                    show_msg(
                                        &run_scan.unwrap_err().to_string(),
                                        MessageLevel::Error,
                                    );
                                    let mut a = self.stop_after.lock().unwrap();
                                    log::debug!("Errors Counter: {}", a);
                                    *a += 1;
                                }
                            }
                        }
                    })
                    .buffer_unordered(self.script_workers)
                    .collect::<Vec<_>>()
            })
            .buffer_unordered(self.workers)
            .collect::<Vec<_>>()
            .await;
    }
}
