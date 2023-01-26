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
use mlua::Lua;
use mlua::ExternalError;
use lua::{parsing::files::filename_to_string, scan::LuaLoader, loader::{encoding_func, get_matching_func, get_utilsfunc, http_func, payloads_func}};

use futures::{stream, StreamExt};
use reqwest::header::HeaderMap;

use glob::glob;
use log::error;

use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};

#[derive(Clone)]
pub struct RequestOpts {
    pub headers: HeaderMap,
    pub proxy: Option<String>,
    pub timeout: u64,
    pub redirects: u32,
}

pub enum ScanTypes {
    URLS,
    HOSTS,
}

pub struct Lotus {
    pub script_path: PathBuf,
    pub output: Option<PathBuf>,
    pub workers: usize,
    pub script_workers: usize,
    pub stop_after: Arc<Mutex<i32>>,
}

impl Lotus {
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
    fn load_scripts(&self) -> Result<Vec<(String, String)>, CliErrors> {
        let mut scripts = Vec::new();
        //
        // Reading one file instead of the dir scripts

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
    fn valid_scripts(&self, scripts: Vec<(String,String)>, number_scantype: bool) -> Vec<(String,String)> {
        let target_url = "http://example.com";
        let lua_eng = Lua::new();
        get_matching_func(&lua_eng);
        http_func(target_url, &lua_eng);
        encoding_func(&lua_eng);
        payloads_func(&lua_eng);
        let mut used_scripts: Vec<(String, String)> = Vec::new();
        scripts.iter().for_each(|(script_code, script_path)| {
            let code = lua_eng.load(script_code).exec();
            if code.is_err() {
            } else {
                let global = lua_eng.globals();
                let scan_type = global.get::<_, usize>("scan_type".to_string());
                if scan_type.is_err() {
                    show_msg(&format!("Unvalid Script Type {}: {}", script_path, scan_type.unwrap_err().to_string()), MessageLevel::Error);
                } else {
                    let scan_type = scan_type.unwrap();
                    if let scan_type = number_scantype {
                        used_scripts.push((script_code.into(), script_path.into()));
                    }
                }
            }
        });
        used_scripts
    }
    pub async fn start(
        &self,
        target_data: Vec<String>,
        request_option: RequestOpts,
        scan_type: ScanTypes,
        exit_after: i32,
    ) {
        let loaded_scripts = {
            if let ScanTypes::HOSTS = scan_type {
                let scripts = self.get_scripts();
                let loaded_scripts = self.valid_scripts(scripts,true);
                loaded_scripts
            } else {
                let scripts = self.get_scripts();
                let loaded_scripts = self.valid_scripts(scripts,false);
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
        stream::iter(target_data)
            .map(move |script_data| {
                let loaded_scripts = loaded_scripts.clone();
                let lotus_loader = Arc::clone(&lotus_obj);
                stream::iter(loaded_scripts.into_iter())
                    .map(move |(script_out, script_name)| {
                        let script_data = script_data.clone();
                        let lotus_loader = Arc::clone(&lotus_loader);
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
                                    .run_scan(script_data.as_str(), None, &script_out, &script_name)
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
