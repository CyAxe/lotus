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

use crate::ScanTypes;
use mlua::Lua;

/// Setup The Lua Runtime
pub struct LuaRunTime<'lua> {
    pub lua: &'lua Lua,
}

pub struct LuaOptions<'a,T: Clone + std::fmt::Display> {
    pub target_url: Option<&'a T>,
    pub target_type: ScanTypes,
    pub fuzz_workers: usize,
    pub script_code: &'a str,
    pub script_dir: &'a str,
    pub env_vars: serde_json::Value,
}
