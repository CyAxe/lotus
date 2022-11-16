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
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use std::collections::HashMap;
mod args;
mod logger;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    if args::cmd_args().is_present("log") == true {
        logger::init_log(args::cmd_args().value_of("log").unwrap()).unwrap();
    }
    let lottas = Lotus::init(args::cmd_args().value_of("scripts").unwrap().to_string());
    let parsed_headers: HashMap<String, String> =
        serde_json::from_str(args::cmd_args().value_of("default_headers").unwrap()).unwrap();
    let mut user_headers = HeaderMap::new();
    user_headers.insert(
        HeaderName::from_bytes("User-agent".as_bytes()).unwrap(),
        HeaderValue::from_bytes(
            "Mozilla/5.0 (X11; Manjaro; Linux x86_64; rv:100.0) Gecko/20100101 Firefox/100.0"
                .as_bytes(),
        )
        .unwrap(),
    );
    parsed_headers.iter().for_each(|(headername, headervalue)| {
        user_headers.insert(
            HeaderName::from_bytes(headername.as_bytes()).unwrap(),
            HeaderValue::from_bytes(headervalue.as_bytes()).unwrap(),
        );
    });
    drop(parsed_headers);
    let request_opts = RequestOpts {
        headers: user_headers,
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
