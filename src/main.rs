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

use lotus::{
    cli::{
        args::Opts,
        bar::{show_msg, MessageLevel},
        errors::CliErrors,
        logger::init_log,
        default_scripts::{FUZZ_EXAMPLE, write_file},
    },
    lua::parsing::files::filename_to_string,
    RequestOpts,
};
use std::{
    io,
    io::BufRead,
    path::PathBuf,
    sync::{Arc, Mutex},
};
use structopt::StructOpt;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let (urls, exit_after, req_opts, lotus_obj) = match Opts::from_args() {
        Opts::URLS {
            redirects,
            workers,
            scripts_workers,
            timeout,
            script_path,
            output,
            proxy,
            log,
            urls,
            headers,
            exit_after,
        } => {
            // setup logger
            init_log(log).unwrap();
            let req_opts = RequestOpts {
                headers,
                proxy,
                timeout,
                redirects,
            };
            let lotus_obj = lotus::Lotus {
                script_path,
                output,
                workers,
                script_workers: scripts_workers,
                stop_after: Arc::new(Mutex::new(1)),
            };
            let urls = get_target_urls(urls);
            (urls, exit_after, req_opts, lotus_obj)
        },
        Opts::NEW { scan_type, file_name } => {
            match scan_type {
                Fuzz => {
                    let write_script_file = write_file(file_name, FUZZ_EXAMPLE);
                    if let Err(CliErrors::FileExists) = write_script_file {
                        show_msg("File Exists, cannot overwrite it, please rename/remove it or try another name", MessageLevel::Error);
                    } else if let Err(CliErrors::WritingError) = write_script_file {
                        show_msg(CliErrors::WritingError.to_string().as_str(), MessageLevel::Error);
                    }
                },
                _ => {
                }
            };
            std::process::exit(0);
        }
    };

    if urls.is_err() {
        match urls {
            Err(CliErrors::EmptyStdin) => {
                show_msg("No input in Stdin", MessageLevel::Error);
            }
            Err(CliErrors::ReadingError) => {
                show_msg("Cannot Read the urls file", MessageLevel::Error);
            }
            _ => {}
        };
        std::process::exit(1);
    }
    lotus_obj
        .start(
            urls.unwrap()
                .iter()
                .map(|url| url.to_string())
                .collect::<Vec<String>>(),
            req_opts,
            exit_after,
        )
        .await;
    Ok(())
}

fn get_target_urls(url_file: Option<PathBuf>) -> Result<Vec<String>, CliErrors> {
    if url_file.is_some() {
        let urls = filename_to_string(url_file.unwrap().to_str().unwrap());
        if urls.is_ok() {
            Ok(urls
                .unwrap()
                .lines()
                .map(|url| url.to_string())
                .collect::<Vec<String>>())
        } else {
            Err(CliErrors::ReadingError)
        }
    } else {
        if atty::is(atty::Stream::Stdin) {
            Err(CliErrors::EmptyStdin)
        } else {
            let stdin = io::stdin();
            let mut urls: Vec<String> = Vec::new();
            stdin.lock().lines().for_each(|x| {
                let the_url = x.unwrap();
                match url::Url::parse(&the_url) {
                    Ok(..) => {
                        urls.push(the_url);
                    }
                    Err(..) => {
                        log::error!("Cannot Parse {} url, ignoring ..", the_url);
                    }
                };
            });
            Ok(urls)
        }
    }
}
