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

use indicatif::{ProgressBar, ProgressStyle};
use std::sync::Mutex;

lazy_static::lazy_static! {
    pub static ref GLOBAL_PROGRESS_BAR: Mutex<Option<ProgressBar>> = Mutex::new(None);
}

pub struct ProgressManager {
    pub progress_bar: ProgressBar,
}

impl ProgressManager {
    pub fn new(size: u64, message: impl Into<String>) -> Self {
        let pb = ProgressBar::new(size);
        pb.set_style(
            ProgressStyle::default_bar()
                .template("{spinner:.green} [{elapsed_precise}] {bar:40.cyan/blue} {pos:>3}/{len:3} {msg}")
                .unwrap()
                .progress_chars("#>-"),
        );
        pb.set_message(message.into());
        *GLOBAL_PROGRESS_BAR.lock().unwrap() = Some(pb.clone());
        ProgressManager { progress_bar: pb }
    }

    pub fn increment(&self, value: u64, message: impl Into<String>) {
        self.progress_bar.inc(value);
        self.progress_bar.set_message(message.into());
    }

    pub fn finish(&self, message: impl Into<String>) {
        self.progress_bar.finish_with_message(message.into());
    }
}
