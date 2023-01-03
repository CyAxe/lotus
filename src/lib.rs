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
use lua::{parsing::files::filename_to_string, scan::LuaLoader};

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

pub struct Lotus {
    pub script_path: PathBuf,
    pub output: Option<PathBuf>,
    pub workers: usize,
    pub script_workers: usize,
    pub stop_after: Arc<Mutex<i32>>,
}

impl Lotus {
    pub async fn start(&self, urls: Vec<String>, request_option: RequestOpts, exit_after: i32) {
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
        let bar =
            create_progress(urls.len() as u64 * loaded_scripts.as_ref().unwrap().len() as u64);
        let loaded_scripts = loaded_scripts.unwrap();
        if self.output.is_none() {
            show_msg("Output argument is missing", MessageLevel::Error);
            std::process::exit(1);
        }
        let lotus_obj = Arc::new(LuaLoader::new(
            &bar,
            request_option.clone(),
            self.output.as_ref().unwrap().to_str().unwrap().to_string(),
        ));
        stream::iter(urls)
            .map(move |url| {
                let loaded_scripts = loaded_scripts.clone();
                let lotus_loader = Arc::clone(&lotus_obj);
                stream::iter(loaded_scripts.into_iter())
                    .map(move |(script_out, script_name)| {
                        let url = url.clone();
                        let lotus_loader = Arc::clone(&lotus_loader);
                        let error_check = {
                            if *self.stop_after.lock().unwrap() == exit_after {
                                log::debug!("Ignoring scripts");
                                false
                            } else {
                                log::debug!("Running {} script on {} url", script_name, url);
                                true
                            }
                        };
                        async move {
                            if error_check == false {
                                // Nothing
                            } else {
                                let run_scan = lotus_loader
                                    .run_scan(url.as_str(), None, &script_out, &script_name)
                                    .await;
                                if run_scan.is_err() {
                                    log::error!("Script is raising error");
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
}
