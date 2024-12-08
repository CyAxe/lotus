/*
 * This file is part of Lotus Project, a Web Security Scanner written in Rust based on Lua Scripts
 * For details, please see https://github.com/rusty-sec/lotus/
 *
 * Copyright (c) 2022 - Khaled Nassar
 *
 * Please note that this file was originally released under the
 * GNU General Public License as published by the Free Software Foundation;
 * either version 2 of the License, or (at your option) any later version.
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use crate::utils::bar::GLOBAL_PROGRESS_BAR;
use chrono::Local;
use colored::*;
use indicatif::ProgressBar;
use log::{Level, Metadata, Record};
use std::sync::Once;

// Ensures logger initialization happens only once during the application lifecycle.
static INIT: Once = Once::new();

pub struct RichLogger;

impl log::Log for RichLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        // Only log messages with a level of Info or lower.
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let time = Local::now().format("%Y-%m-%d %H:%M:%S"); // Timestamp for log messages.
            let log_level = match record.level() {
                Level::Info => "INFO".bright_green(),
                Level::Warn => "WARN".bright_yellow(),
                Level::Error => "ERROR".bright_red(),
                _ => "LOG".normal(),
            };
            let formatted_message = format!(
                "[{}] [{}] {}",
                time.to_string().bright_blue(),
                log_level,
                record.args()
            );

            // Print log messages to the progress bar if it exists, otherwise to the console.
            let progress_bar = GLOBAL_PROGRESS_BAR.lock().unwrap();
            if let Some(ref pb) = *progress_bar {
                pb.println(formatted_message);
            } else {
                println!("{}", formatted_message);
            }
        }
    }

    fn flush(&self) {}
}

/// Initializes the custom logger with a shared progress bar.
/// Ensures logging integrates seamlessly with progress tracking.
pub fn init_logger(progress_bar: ProgressBar) {
    INIT.call_once(|| {
        *GLOBAL_PROGRESS_BAR.lock().unwrap() = Some(progress_bar);
        log::set_logger(&RichLogger).unwrap();
        log::set_max_level(log::LevelFilter::Info);
    });
}

// Macro for logging informational messages with a consistent format.
#[macro_export]
macro_rules! log_info {
    ($($arg:tt)*) => ({
        log::info!($($arg)*);
    })
}

// Macro for logging warning messages with a consistent format.
#[macro_export]
macro_rules! log_warn {
    ($($arg:tt)*) => ({
        log::warn!($($arg)*);
    })
}

// Macro for logging error messages with a consistent format.
#[macro_export]
macro_rules! log_error {
    ($($arg:tt)*) => ({
        log::error!($($arg)*);
    })
}
