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

use lotus::Lotus;
use lotus::RequestOpts;
mod args;
mod logger;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    if args::cmd_args().is_present("log") == true {
        logger::init_log(args::cmd_args().value_of("log").unwrap()).unwrap();
    }
    let lottas = Lotus::init(args::cmd_args().value_of("scripts").unwrap().to_string());
    let request_opts = RequestOpts {
        proxy: match args::cmd_args().value_of("proxy") {
            Some(proxy) => Some(proxy.to_string()),
            None => None,
        },
        timeout: args::cmd_args()
            .value_of("timeout")
            .unwrap()
            .to_string()
            .parse::<u64>()
            .unwrap(),
        redirects: args::cmd_args()
            .value_of("redirects")
            .unwrap()
            .to_string()
            .parse::<u32>()
            .unwrap(),
    };
    lottas
        .start(
            args::cmd_args()
                .value_of("workers")
                .unwrap()
                .trim()
                .parse::<usize>()
                .unwrap(),
            request_opts,
            args::cmd_args()
                .value_of("script_threads")
                .unwrap()
                .trim()
                .parse::<usize>()
                .unwrap(),
            &args::cmd_args().value_of("output").unwrap_or(""),
            &args::cmd_args().value_of("out_script").unwrap_or(""),
        )
        .await;
    Ok(())
}
