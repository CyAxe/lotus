// This file is part of Lotus Project, a web security scanner written in Rust based on Lua scripts.
// For details, please see https://github.com/rusty-sec/lotus/
//
// Copyright (c) 2022 - Khaled Nassar
//
// Please note that this file was originally released under the GNU General Public License as
// published by the Free Software Foundation; either version 2 of the License, or (at your option)
// any later version.
//
// Unless required by applicable law or agreed to in writing, software distributed under the
// License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND,
// either express or implied. See the License for the specific language governing permissions
// and limitations under the License.

pub mod cli;
pub mod lua;

use cli::{
    bar::{show_msg, MessageLevel, BAR},
    errors::CliErrors,
    input::load_scripts::{get_scripts, valid_scripts},
};
use lua::{
    loader::{LuaOptions, LuaRunTime},
    parsing::files::filename_to_string,
    run::LuaLoader,
    threads::runner::iter_futures,
};
use reqwest::header::HeaderMap;

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
    /// HOSTS Scanning under ID number 1
    HOSTS,
    /// URLS Scanning under ID number 2
    URLS,
    /// PATHS Scanning under ID number 3
    PATHS,
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
    pub env_vars: serde_json::Value
}

impl Lotus {
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
        fuzz_workers: usize,
    ) {
        let loaded_scripts = match scan_type {
            ScanTypes::HOSTS => valid_scripts(get_scripts(self.script_path.clone()), 1),
            ScanTypes::PATHS => valid_scripts(get_scripts(self.script_path.clone()), 3),
            _ => valid_scripts(get_scripts(self.script_path.clone()), 2),
        };
        if self.output.is_none() {
            show_msg("Output argument is missing", MessageLevel::Error);
            std::process::exit(1);
        }
        let lotus_obj = Arc::new(LuaLoader::new(
            request_option.clone(),
            self.output.clone(),
        ));
        let scan_type = Arc::new(scan_type);
        iter_futures(
            target_data,
            |script_data| async move {
                let lotus_loader = Arc::clone(&lotus_obj);
                let scan_type = Arc::clone(&scan_type);
                iter_futures(
                    loaded_scripts,
                    |(script_code, script_name)| async move {
                        let lua_opts = LuaOptions {
                            target_url: Some(&script_data),
                            target_type: *scan_type,
                            fuzz_workers,
                            script_code: &script_code,
                            script_dir: &script_name,
                            env_vars: self.env_vars.clone()
                        };
                        if *self.stop_after.lock().unwrap() == exit_after {
                            log::debug!("Ignoring scripts");
                        } else {
                            log::debug!("Running {} script on {}", script_name, script_data);
                            match lotus_loader.run_scan(lua_opts).await {
                                Ok(_) => (),
                                Err(err) => {
                                    log::error!("script error: {}", err);
                                    let mut stop_after = self.stop_after.lock().unwrap();
                                    log::debug!("Errors Counter: {}", *stop_after);
                                    *stop_after += 1;
                                }
                            }
                        }
                    },
                    self.script_workers,
                )
                .await;
            },
            self.workers,
        )
        .await;
    }
}
