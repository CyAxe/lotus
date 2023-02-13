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

use console::Style;
use lazy_static::lazy_static;
use indicatif::{ProgressBar, ProgressStyle};
use std::sync::{Arc, Mutex};

pub enum MessageLevel {
    Info,
    Warn,
    Error,
}

lazy_static! {
    pub static ref BAR: Arc<Mutex<ProgressBar>> = Arc::new(Mutex::new(ProgressBar::new(0)));
}

/// Lotus ProgressBar based on the length of `bar` parameter
pub fn create_progress(bar: u64) {
    let bar_length = BAR.lock().unwrap().length().unwrap();
    BAR.lock().unwrap().set_length(bar + bar_length);
    BAR.lock().unwrap().set_style(
        ProgressStyle::default_bar()
            .template(
                "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos:>7}/{len:7} {msg}",
            )
            .expect("ProgressBar Error")
            .tick_chars("⣾⣽⣻⢿⡿⣟⣯⣷".to_string().as_str())
            .progress_chars("#>-"),
    );
}

pub fn show_msg(message: &str, msglevel: MessageLevel) {
    let print_level = match msglevel {
        MessageLevel::Info => {
            log::info!("{}", message);
            format!("[{}]", Style::new().blue().apply_to("INFO"))
        }
        MessageLevel::Warn => {
            log::warn!("{}", message);
            format!("[{}]", Style::new().yellow().apply_to("WARN"))
        }
        MessageLevel::Error => {
            log::error!("{}", message);
            format!("[{}]", Style::new().red().apply_to("ERROR"))
        }
    };
    if let MessageLevel::Error = msglevel {
        eprintln!("{print_level} {message}");
    } else {
        println!("{print_level} {message}");
    }
}
