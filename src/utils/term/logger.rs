use chrono::Local;
use colored::*;
use log::{info, warn, error, Record, Level, Metadata};
use std::sync::{Once, Mutex};
use indicatif::ProgressBar;

static INIT: Once = Once::new();
static GLOBAL_PROGRESS_BAR: Mutex<Option<ProgressBar>> = Mutex::new(None);

pub struct RichLogger;

impl log::Log for RichLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let time = Local::now().format("%Y-%m-%d %H:%M:%S");
            let log_level = match record.level() {
                Level::Info => "INFO".bright_green(),
                Level::Warn => "WARN".bright_yellow(),
                Level::Error => "ERROR".bright_red(),
                _ => "LOG".normal(),
            };
            let formatted_message = format!("[{}] [{}] {}", time.to_string().bright_blue(), log_level, record.args());

            let mut progress_bar = GLOBAL_PROGRESS_BAR.lock().unwrap();
            if let Some(ref pb) = *progress_bar {
                pb.println(formatted_message);
            } else {
                println!("{}", formatted_message);
            }
        }
    }

    fn flush(&self) {}
}

pub fn init_logger(progress_bar: ProgressBar) {
    INIT.call_once(|| {
        *GLOBAL_PROGRESS_BAR.lock().unwrap() = Some(progress_bar);
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
