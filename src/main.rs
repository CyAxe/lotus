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

use lotus::cli::{
    args::Opts,
    startup::{new::new_args, urls::args_urls},
};
use structopt::StructOpt;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    match Opts::from_args() {
        Opts::URLS { .. } => {
            let opts = args_urls();
            opts.lotus_obj
                .start(
                    opts.urls
                        .iter()
                        .map(|url| url.to_string())
                        .collect::<Vec<String>>(),
                    opts.req_opts,
                    opts.exit_after,
                )
                .await;
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
