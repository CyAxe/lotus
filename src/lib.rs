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
    bar::{show_msg, MessageLevel, BAR},
    errors::CliErrors,
    input::load_scripts::{get_scripts, valid_scripts},
};
use lua::{
    loader::LuaRunTime,
    parsing::files::filename_to_string,
    scan::LuaLoader,
    threads::runner::{iter_futures, iter_futures_tuple},
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
        fuzz_workers: usize
    ) {
        let loaded_scripts = {
            if let ScanTypes::HOSTS = scan_type {
                let scripts = get_scripts(self.script_path.clone());
                let loaded_scripts = valid_scripts(scripts, 1);
                log::debug!("Running Host scan {:?}", loaded_scripts.len());
                loaded_scripts
            } else if let ScanTypes::PATHS = scan_type {
                let scripts = get_scripts(self.script_path.clone());
                let loaded_scripts = valid_scripts(scripts, 3);
                log::debug!("Running PATH scan {:?}", loaded_scripts.len());
                loaded_scripts
            } else {
                let scripts = get_scripts(self.script_path.clone());
                let loaded_scripts = valid_scripts(scripts, 2);
                log::debug!("Running URL scan {:?}", loaded_scripts.len());
                loaded_scripts
            }
        };
        if self.output.is_none() {
            show_msg("Output argument is missing", MessageLevel::Error);
            std::process::exit(1);
        }
        let lotus_obj = Arc::new(LuaLoader::new(
            request_option.clone(),
            self.output.as_ref().unwrap().to_str().unwrap().to_string(),
        ));
        let scan_type = Arc::new(scan_type);
        iter_futures(
            target_data.clone(),
            |script_data| async move {
                let loaded_scripts = loaded_scripts.clone();
                let lotus_loader = Arc::clone(&lotus_obj);
                let scan_type = Arc::clone(&scan_type);
                iter_futures_tuple(
                    loaded_scripts,
                    |(script_code, script_name)| async move {
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
                        if error_check == false {
                            // Nothing
                        } else {
                            let run_scan = lotus_loader
                                .run_scan(
                                    Some(script_data.as_str()),
                                    scan_type,
                                    fuzz_workers,
                                    &script_code,
                                    &script_name,
                                )
                                .await;
                            if run_scan.is_err() {
                                log::error!(
                                    "script error: {}",
                                    &run_scan.clone().unwrap_err().to_string()
                                );
                                show_msg(&run_scan.unwrap_err().to_string(), MessageLevel::Error);
                                let mut a = self.stop_after.lock().unwrap();
                                log::debug!("Errors Counter: {}", a);
                                *a += 1;
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
