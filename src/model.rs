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
#[derive(Clone, Copy)]
pub enum ScanTypes {
    /// HOSTS Scanning under ID number 1
    HOSTS,
    /// URLS Scanning under ID number 2
    URLS,
    /// PATHS Scanning under ID number 3
    PATHS,
    /// CUSTOM Scannig under ID number 4
    CUSTOM,
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
