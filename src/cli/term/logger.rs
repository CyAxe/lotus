use chrono::Local;
use log::{Record, Level, Metadata};
use std::sync::Once;
use indicatif::ProgressBar;

static INIT: Once = Once::new();

pub struct RichLogger;

impl log::Log for RichLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let time = Local::now().format("%Y-%m-%d %H:%M:%S");
            let message = match record.level() {
                Level::Info => format!("{} {}", time.to_string(), record.args().to_string()),
                Level::Warn => format!("{} {}", time.to_string(), record.args().to_string()),
                Level::Error => format!("{} {}", time.to_string(), record.args().to_string()),
                _ => format!("{} {}", time.to_string(), record.args().to_string())
            };

            let progress_bar = ProgressBar::new_spinner();
            progress_bar.println(message);
        }
    }

    fn flush(&self) {}
}

pub fn init_logger() {
    INIT.call_once(|| {
        log::set_logger(&RichLogger).unwrap();
        log::set_max_level(log::LevelFilter::Info);
    });
}

#[macro_export]
macro_rules! log_info {
    ($($arg:tt)*) => ({
        log::info!($($arg)*);
    })
}

#[macro_export]
macro_rules! log_warn {
    ($($arg:tt)*) => ({
        log::warn!($($arg)*);
    })
}

#[macro_export]
macro_rules! log_error {
    ($($arg:tt)*) => ({
        log::error!($($arg)*);
    })
}

