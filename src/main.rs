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
use cli::args::Opts;
use cli::errors::CliErrors;
use cli::logger::init_log;
use lotus::RequestOpts;
use std::io;
use std::io::BufRead;
use std::sync::{Mutex, Arc};
use structopt::StructOpt;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let args = Opts::from_args();
    // setup logger
    init_log(args.log).unwrap();

    let urls = get_target_urls();
    if urls.is_err() {
        eprintln!("EmptyStdin");
        std::process::exit(1);
    }
    // default request options
    let req_opts = RequestOpts {
        headers: args.headers,
        proxy: args.proxy,
        timeout: args.timeout,
        redirects: args.redirects,
    };
    let lotus_obj = lotus::Lotus {
        script_path: args.script_path,
        output: args.output,
        workers: args.workers,
        script_workers: args.scripts_workers,
        stop_after: Arc::new(Mutex::new(1))
    };
    lotus_obj
        .start(
            urls.unwrap()
                .iter()
                .map(|url| url.to_string())
                .collect::<Vec<String>>(),
            req_opts,
            args.exit_after
        )
        .await;
    Ok(())
}

fn get_target_urls() -> Result<Vec<String>, CliErrors> {
    if atty::is(atty::Stream::Stdin) {
        Err(CliErrors::EmptyStdin)
    } else {
        let stdin = io::stdin();
        Ok(stdin
            .lock()
            .lines()
            .map(|x| {
                let the_url = x.unwrap();
                match url::Url::parse(&the_url) {
                    Ok(_url) => {}
                    Err(_err) => {}
                }
                the_url
            })
            .collect::<Vec<String>>())
    }
}
