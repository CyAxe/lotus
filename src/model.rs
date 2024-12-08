use reqwest::header::HeaderMap;
use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};

/// Lotus HTTP Options
#[derive(Clone)]
pub struct RequestOpts {
    /// Default Headers
    pub headers: HeaderMap,
    /// Custom http Proxy
    pub proxy: Option<String>,
    /// Request Timeout
    pub timeout: u64,
    /// Limits of http redirects
    pub redirects: u32,
}

/// Scanning type For `SCAN_TYPE` in lua scripts
#[derive(Debug, Clone, Copy)]
pub enum ScanTypes {
    /// FULL_HTTP Scanning
    FULL_HTTP,
    /// HOSTS Scanning under ID number 1
    HOSTS,
    /// URLS Scanning under ID number 2
    URLS,
    /// PATHS Scanning under ID number 3
    PATHS,
    /// CUSTOM Scanning under ID number 4
    CUSTOM,
}

impl ScanTypes {
    /// Converts the enum variant into its string representation.
    pub fn as_str(&self) -> &'static str {
        match self {
            ScanTypes::FULL_HTTP => "FULL_HTTP",
            ScanTypes::HOSTS => "HOSTS",
            ScanTypes::URLS => "URLS",
            ScanTypes::PATHS => "PATHS",
            ScanTypes::CUSTOM => "CUSTOM",
        }
    }
}

pub struct Lotus {
    /// Script Path
    pub script_path: PathBuf,
    /// Output Path
    pub output: Option<PathBuf>,
    /// Workers Option
    pub workers: usize,
    /// How many url per script
    pub script_workers: usize,
    /// Stop After X of errors
    pub stop_after: Arc<Mutex<i32>>,
    pub env_vars: serde_json::Value,
}
