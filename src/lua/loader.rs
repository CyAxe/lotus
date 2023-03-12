/*
 * This file is part of Lotus Project, an Web Security Scanner written in Rust based on Lua Scripts
 * For details, please see https://github.com/rusty-sec/lotus/
 *
 * Copyright (c) 2022 - Khaled Nassar
 *
 * Please note that this file was originally released under the
 * GNU General Public License as ished by the Free Software Foundation;
 * either version 2 of the License, or (at your option) any later version.
 *
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use crate::cli::errors::CliErrors;
use crate::ScanTypes;
use log::error;
use mlua::Lua;

/// Setup The Lua Runtime
pub struct LuaRunTime<'lua> {
    pub lua: &'lua Lua,
}

pub struct LuaOptions<'a> {
    pub target_url: Option<&'a str>,
    pub target_type: ScanTypes,
    pub fuzz_workers: usize,
    pub script_code: &'a str,
    pub script_dir: &'a str
}

/// check if the regex pattern is matching with this string or not without get the matched parts
pub fn is_match(pattern: String, resp: String) -> Result<bool, CliErrors> {
    let re = fancy_regex::Regex::new(&pattern);
    if let Ok(..) = re {
        let matched = re.unwrap().is_match(&resp);
        if matched.is_err() {
            error!("Cannot match with resp value: {}", resp);
            Err(CliErrors::RegexError)
        } else {
            Ok(matched.unwrap())
        }
    } else {
        error!("Regex Pattern ERROR  {:?}", pattern);
        Err(CliErrors::RegexPatternError)
    }
}
