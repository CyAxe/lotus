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
        bar::create_progress,
        startup::{new::new_args, urls::args_urls},
    },
    lua::{
        network::http::{REQUESTS_LIMIT, SLEEP_TIME},
        threads::runner,
    },
    ScanTypes,
};
use structopt::StructOpt;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    match Opts::from_args() {
        Opts::SITEMAP { .. } => {}
        Opts::URLS { .. } => {
            let opts = args_urls();
            // Open two threads for URL/HOST scanning
            create_progress((opts.target_data.urls.len() * opts.target_data.hosts.len()) as u64);
            *SLEEP_TIME.lock().unwrap() = opts.delay;
            *REQUESTS_LIMIT.lock().unwrap() = opts.requests_limit;
            let scan_futures = vec![
                opts.lotus_obj.start(
                    opts.target_data.paths,
                    opts.req_opts.clone(),
                    ScanTypes::PATHS,
                    opts.exit_after,
                ),
                opts.lotus_obj.start(
                    opts.target_data.urls,
                    opts.req_opts.clone(),
                    ScanTypes::URLS,
                    opts.exit_after,
                ),
                opts.lotus_obj.start(
                    opts.target_data.hosts,
                    opts.req_opts,
                    ScanTypes::HOSTS,
                    opts.exit_after,
                ),
            ];
            runner::scan_futures(scan_futures, 3, None).await;
        }
        Opts::NEW {
            scan_type,
            file_name,
        } => {
            new_args(scan_type, file_name);
            std::process::exit(0);
        }
    };
    Ok(())
}
