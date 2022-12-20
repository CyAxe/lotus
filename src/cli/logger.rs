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

// Lotus init logger
use std::path::PathBuf;

pub fn init_log(log_file: Option<PathBuf>) -> Result<(), std::io::Error> {
    let logger = fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .level_for("hyper", log::LevelFilter::Warn)
        .level_for("reqwest", log::LevelFilter::Warn)
        .level_for("isahc", log::LevelFilter::Warn)
        .level_for("selectors", log::LevelFilter::Warn)
        .level_for("html5ever", log::LevelFilter::Warn);
    match log_file {
        Some(log_path) => {
            // Disalbe unwanted loggers
            logger
                .chain(fern::log_file(log_path).unwrap())
                .apply()
                .unwrap();
        }
        None => {}
    }
    Ok(())
}
