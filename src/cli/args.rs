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

use clap::{App, Arg, ArgMatches};
mod validator;
pub fn cmd_args() -> ArgMatches {
    App::new("Lotus")
        .version("0.3-beta")
        .author("Khaled Nassar <knassar702@gmail.com>")
        .about("Fast Web Security Scanner written in Rust based on Lua Scripts ")
        .arg(
            Arg::with_name("out_script")
                .help("Use Custom Lua for reporting")
                .long("--lua-report")
                .validator(validator::file_exists)
                .takes_value(true)
                .required_unless_present("output"),
        )
        .arg(
            Arg::with_name("default_headers")
                .help("Default Request Headers")
                .validator(validator::valid_json)
                .takes_value(true)
                .default_value("{}")
                .long("headers"),
        )
        .arg(
            Arg::with_name("redirects")
                .help("Set limit of http redirects")
                .long("redirects")
                .default_value("10"),
        )
        .arg(
            Arg::with_name("timeout")
                .help("Set Connection timeout")
                .long("timeout")
                .default_value("10"),
        )
        .arg(
            Arg::with_name("proxy")
                .help("Forward all connection through a proxy (eg: --proxy http://localhost:8080)")
                .required(false)
                .takes_value(true)
                .validator(url::Url::parse)
                .long("proxy"),
        )
        .arg(
            Arg::with_name("workers")
                .help("Number of works of urls")
                .short('w')
                .takes_value(true)
                .default_value("10")
                .long("workers"),
        )
        .arg(
            Arg::with_name("log")
                .help("Save all logs to custom file")
                .takes_value(true)
                .short('l')
                .long("log"),
        )
        .arg(
            Arg::with_name("script_threads")
                .help("Workers for lua scripts")
                .short('t')
                .long("script-threads")
                .takes_value(true)
                .default_value("5"),
        )
        .arg(
            Arg::with_name("scripts")
                .help("Path of scripts dir")
                .takes_value(true)
                .short('s')
                .long("scripts")
                .required(true),
        )
        .arg(
            Arg::with_name("output")
                .help("Path of the JSON output fiel")
                .required_if_eq("out_script", "")
                .takes_value(true)
                .long("output")
                .short('o'),
        )
        .arg(Arg::with_name("nolog").help("no logging"))
        .get_matches()
}
