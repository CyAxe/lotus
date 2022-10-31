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

use log::debug;
pub mod bar;
pub mod files;
pub mod html;
pub mod http;
// pub mod oast;
pub mod report;
pub mod url;

/// check if the regex pattern is matching with this string or not without get the matched parts
/// you can use it for sqli errors for example
/// ```rust
/// assert_eq!(true,is_match("\d","1"));
/// assert_eq!(false,is_match("\d\d","1"))
/// ```
pub fn is_match(pattern: String, resp: String) -> bool {
    let re = fancy_regex::Regex::new(&pattern);
    if let Ok(..) = re {
        re.unwrap().is_match(&resp).unwrap_or(false)
    } else {
        debug!("MATCHING ERROR  {:?} | {:?}", pattern, re);
        false
    }
}
