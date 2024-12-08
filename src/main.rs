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

use lotus::cli::{
        args::Opts,
        startup::new::new_args,
    };
use structopt::StructOpt;
mod starter;
use starter::run_scan;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    if let Opts::NEW(new_opts) = Opts::from_args() {
        new_args(new_opts.scan_type, new_opts.file_name);
        std::process::exit(0);
    }
    run_scan().await.unwrap();
    Ok(())
}
