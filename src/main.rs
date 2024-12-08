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

// The `starter` module contains the core functionality required to initiate a scan.
mod starter;

use starter::run_scan;

// Main entry point of the Lotus Project.
#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // Call the `run_scan` function to execute the main scanning logic.
    run_scan().await.unwrap();

    Ok(())
}
