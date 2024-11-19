use console::{Emoji};
use indicatif::{ProgressBar, ProgressStyle};
use std::sync::Mutex;

static SPARKLE: Emoji<'_, '_> = Emoji("✨", "");
static FIRE: Emoji<'_, '_> = Emoji("🔥", "");

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
