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
        args::{Opts, ScriptType},
        input::get_target_urls,
        bar::{show_msg, MessageLevel},
        default_scripts::{
            write_file, CVE_EXAMPLE, FUZZ_EXAMPLE, PASSIVE_EXAMPLE, SERVICE_EXAMPLE,
        },
        errors::CliErrors,
        logger::init_log,
    },
    RequestOpts,
};
use structopt::StructOpt;
use std::sync::{Mutex, Arc};

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
        }
        Opts::NEW {
            scan_type,
            file_name,
        } => {
            let script_code = match scan_type {
                ScriptType::Fuzz => FUZZ_EXAMPLE,
                ScriptType::CVE => CVE_EXAMPLE,
                ScriptType::PASSIVE => PASSIVE_EXAMPLE,
                ScriptType::SERVICE => SERVICE_EXAMPLE,
                ScriptType::NotSupported => "",
            };
            let write_script_file = write_file(file_name, script_code);
            if let Err(CliErrors::FileExists) = write_script_file {
                show_msg(
                    "File Exists, cannot overwrite it, please rename/remove it or try another name",
                    MessageLevel::Error,
                );
            } else if let Err(CliErrors::WritingError) = write_script_file {
                show_msg(
                    CliErrors::WritingError.to_string().as_str(),
                    MessageLevel::Error,
                );
            } else {
                show_msg(
                    "A copy of the Example file has been created",
                    MessageLevel::Info,
                );
            }
            show_msg("Exit ..", MessageLevel::Info);
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

