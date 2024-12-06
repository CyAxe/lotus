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
pub mod utils;
mod model;

use cli::{
    errors::CliErrors,
    input::load_scripts::valid_scripts,
};
use lua::{
    model::LuaOptions,
    parsing::files::filename_to_string,
    run::LuaLoader,
    threads::runner::{iter_futures, LAST_HOST_SCAN_ID, LAST_HTTP_SCAN_ID},
};
use mlua::ExternalError;
pub use model::{Lotus, RequestOpts, ScanTypes};
use std::sync::Arc;

use crate::lua::threads::runner::{LAST_CUSTOM_SCAN_ID, LAST_PATH_SCAN_ID, LAST_URL_SCAN_ID};

impl Lotus {
    /// Run the Lua script with real target.
    ///
    /// * `target_data` - Vector with target urls.
    /// * `loaded_scripts` - Loaded Lua scripts.
    /// * `request_option` - RequestOpts contains some HTTP request options like (proxy, timeout).
    /// * `scan_type` - Scan type if it's host or URL scanning.
    /// * `exit_after` - Exit after how many errors.
    /// * `fuzz_workers` - Number of fuzz workers.
    pub async fn start(
        &self,
        target_data: Vec<serde_json::Value>,
        loaded_scripts: Vec<(String, String)>,
        request_option: RequestOpts,
        scan_type: ScanTypes,
        exit_after: i32,
        fuzz_workers: usize,
    ) {
        if target_data.is_empty() {
            return;
        }

        let resume_value: usize;
        let loaded_scripts = match scan_type {
            ScanTypes::FULL_HTTP => {
                resume_value = *LAST_HTTP_SCAN_ID.lock().await;
                valid_scripts(loaded_scripts, 0)
            }
            ScanTypes::HOSTS => {
                resume_value = *LAST_HOST_SCAN_ID.lock().await;
                valid_scripts(loaded_scripts, 1)
            }
            ScanTypes::URLS => {
                resume_value = *LAST_URL_SCAN_ID.lock().await;
                valid_scripts(loaded_scripts, 2)
            }
            ScanTypes::PATHS => {
                resume_value = *LAST_PATH_SCAN_ID.lock().await;
                valid_scripts(loaded_scripts, 3)
            }
            ScanTypes::CUSTOM => {
                resume_value = *LAST_CUSTOM_SCAN_ID.lock().await;
                valid_scripts(loaded_scripts, 4)
            }
        };

        let lotus_obj = Arc::new(LuaLoader::new(request_option.clone(), self.output.clone()));
        let scan_type = Arc::new(scan_type);

        iter_futures(
            scan_type.clone(),
            target_data,
            |script_data| async move {
                let lotus_loader = Arc::clone(&lotus_obj);
                let scan_type = Arc::clone(&scan_type);

                iter_futures(
                    scan_type.clone(),
                    loaded_scripts.clone(),
                    |(script_code, script_name)| async move {
                        let lua_opts = LuaOptions {
                            target_url: Some(&script_data),
                            target_type: *scan_type,
                            fuzz_workers,
                            script_code: &script_code,
                            script_dir: &script_name,
                            env_vars: self.env_vars.clone(),
                        };

                        let stop_after: i32 = {*self.stop_after.lock().unwrap()};
                        if stop_after == exit_after{
                           // No script will be executed
                        } else {
                            log::debug!("Starting script execution: {} on {}", script_name, script_data);
                            match lotus_loader.run_scan(lua_opts).await {
                                Ok(_) => (),
                                Err(err) => {
                                    log::error!("An error occurred while executing the script: {}", err.to_lua_err().to_string());
                                    {
                                        let mut stop_after = self.stop_after.lock().unwrap(); log::debug!("The current number of errors encountered while executing scripts is: {}", *stop_after);
                                        *stop_after += 1;
                                    }
                                }
                            }
                        }
                    },
                    self.script_workers,
                    0,
                    false,
                )
                .await;
            },
            self.workers,
            resume_value,
            true,
        )
        .await;
    }
}
