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

mod cli;
mod parsing;
mod scan;
mod lua_api;
mod network;
mod payloads;
mod output;

use cli::errors::CliErrors;
use glob::glob;
use log::error;
use parsing::files::filename_to_string;
use cli::bar::create_progress;
use reqwest::header::HeaderMap;
use std::path::PathBuf;

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
}

impl Lotus {
    pub async fn start(&self, urls: Vec<String>, request_option: RequestOpts) {
        let loaded_scripts = {
                if self.script_path.is_dir() {
                    self.load_scripts()
                } else {
                    self.load_script()
                }
        };
        if loaded_scripts.is_err() {
            eprintln!("Reading errors");// TODO
            std::process::exit(1);
        }
        let bar = create_progress(urls.len() as u64 * loaded_scripts.as_ref().unwrap().len() as u64);
        if loaded_scripts.is_err() {
            eprintln!("Reading error bruh"); // TODO
            std::process::exit(1);
        }

        for target_script in loaded_scripts.unwrap() {
            for url in &urls {
                let lotus_obj = scan::LuaLoader::new(&bar,request_option.clone(), self.output.as_ref().unwrap().to_str().unwrap().to_string());
                lotus_obj.run_scan(&url,None,&target_script.0,&target_script.1).await;
            }
        }
    }

    fn load_scripts(&self) -> Result<Vec<(String, String)>, CliErrors> {
        let mut scripts = Vec::new();
        //
        // Reading one file instead of the dir scripts

        for entry in
            glob(format!("{}{}", self.script_path.to_str().unwrap(), "/*.lua").as_str())
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
            return Err(CliErrors::ReadingError);
        } else {
            scripts.push((
                read_script_code.unwrap(),
                script_path.to_str().unwrap().to_string(),
            ));
            return Ok(scripts);
        }
    }
}
