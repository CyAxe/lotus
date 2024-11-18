use indicatif::{ProgressBar, ProgressStyle};

pub struct ProgressManager {
    progress_bar: ProgressBar,
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
