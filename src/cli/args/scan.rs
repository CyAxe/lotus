use crate::cli::errors::CliErrors;
use crate::cli::input::parse_requests::{InjectionLocation, SCAN_CONTENT_TYPE};
use crate::lua::threads::runner::{
    LAST_CUSTOM_SCAN_ID, LAST_HOST_SCAN_ID, LAST_HTTP_SCAN_ID, LAST_PATH_SCAN_ID, LAST_URL_SCAN_ID,
};
use futures::executor::block_on;
use reqwest::header::HeaderMap;
use reqwest::header::{HeaderName, HeaderValue};
use serde_json::Value;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use structopt::StructOpt;

/// Parses the scan content type from a comma-separated string.
/// Updates a global SCAN_CONTENT_TYPE list based on valid input types.
fn parse_scan_content_type(input_content: &str) -> Result<(), CliErrors> {
    let mut is_error = false;
    // Clear previous scan types.
    block_on(async { SCAN_CONTENT_TYPE.lock().await.clear() });

    input_content.split(',').for_each(|the_scan_type| {
        match the_scan_type {
            "url" => block_on(async { SCAN_CONTENT_TYPE.lock().await.push(InjectionLocation::Url) }),
            "body" => block_on(async { SCAN_CONTENT_TYPE.lock().await.push(InjectionLocation::Body) }),
            "json" => block_on(async {
                SCAN_CONTENT_TYPE.lock().await.push(InjectionLocation::BodyJson)
            }),
            "headers" => block_on(async {
                SCAN_CONTENT_TYPE.lock().await.push(InjectionLocation::Headers)
            }),
            _ => is_error = true,
        }
    });

    if is_error {
        Err(CliErrors::UnsupportedScanType)
    } else {
        Ok(())
    }
}

/// Reads a resume configuration file and updates scan IDs accordingly.
fn read_resume_file(file_path: &str) -> Result<(), std::io::Error> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split('=').collect();
        if parts.len() != 2 {
            continue;
        }

        match parts[0] {
            "HTTP_SCAN_ID" => {
                let mut scan_id = block_on(LAST_HTTP_SCAN_ID.lock());
                *scan_id = parts[1].parse().unwrap_or(0);
            }
            "URL_SCAN_ID" => {
                let mut scan_id = block_on(LAST_URL_SCAN_ID.lock());
                *scan_id = parts[1].parse().unwrap_or(0);
            }
            "HOST_SCAN_ID" => {
                let mut scan_id = block_on(LAST_HOST_SCAN_ID.lock());
                *scan_id = parts[1].parse().unwrap_or(0);
            }
            "PATH_SCAN_ID" => {
                let mut scan_id = block_on(LAST_PATH_SCAN_ID.lock());
                *scan_id = parts[1].parse().unwrap_or(0);
            }
            "CUSTOM_SCAN_ID" => {
                let mut scan_id = block_on(LAST_CUSTOM_SCAN_ID.lock());
                *scan_id = parts[1].parse().unwrap_or(0);
            }
            _ => {}
        }
    }

    Ok(())
}

/// Parses custom HTTP headers from a JSON string.
/// Returns a HeaderMap populated with default and user-defined headers.
fn parse_headers(raw_headers: &str) -> Result<HeaderMap, serde_json::Error> {
    let parsed_json = serde_json::from_str::<HashMap<String, String>>(raw_headers);
    if parsed_json.is_err() {
        return Err(parsed_json.unwrap_err());
    }

    let mut user_headers = HeaderMap::new();
    user_headers.insert(
        HeaderName::from_bytes("User-agent".as_bytes()).unwrap(),
        HeaderValue::from_bytes(
            "Mozilla/5.0 (X11; Manjaro; Linux x86_64; rv:100.0) Gecko/20100101 Firefox/100.0"
                .as_bytes(),
        )
        .unwrap(),
    );

    parsed_json.unwrap().iter().for_each(|(headername, headervalue)| {
        user_headers.insert(
            HeaderName::from_bytes(headername.as_bytes()).unwrap(),
            HeaderValue::from_bytes(headervalue.as_bytes()).unwrap(),
        );
    });

    Ok(user_headers)
}

/// Parses environment variables from a JSON string.
/// Returns the parsed variables as a serde_json Value.
fn get_env_vars(env_vars_json: &str) -> Result<Value, serde_json::Error> {
    let parsed_vars = serde_json::from_str(env_vars_json)?;
    Ok(parsed_vars)
}

#[derive(Debug, StructOpt)]
pub struct UrlsOpts {
    /// Maximum number of allowed HTTP redirects.
    #[structopt(short, long, default_value = "10", help = "Number of allowed HTTP redirects")]
    pub redirects: u32,

    /// Number of workers for fuzzing processes.
    #[structopt(long = "fuzz-workers", default_value = "15", help = "Number of fuzzing workers")]
    pub fuzz_workers: usize,

    /// Scan content types to handle.
    #[structopt(
        short,
        long = "content-type",
        default_value = "url,body,headers",
        parse(try_from_str = parse_scan_content_type)
    )]
    pub _content_type: (),

    /// Number of concurrent workers.
    #[structopt(short = "w", long = "workers", default_value = "10", help = "Number of workers")]
    pub workers: usize,

    /// Enable verbose mode to show request details.
    #[structopt(short = "v", long = "verbose", help = "Enable verbose mode")]
    pub verbose: bool,

    /// Number of scripts to execute concurrently for each URL.
    #[structopt(
        short = "sw",
        long = "scripts-worker",
        default_value = "10",
        help = "Number of concurrent scripts per URL"
    )]
    pub scripts_workers: usize,

    /// Connection timeout in seconds.
    #[structopt(short = "t", long = "timeout", default_value = "10", help = "Connection timeout")]
    pub timeout: u64,

    /// Path to the script directory.
    #[structopt(parse(from_os_str), help = "Path to the script directory")]
    pub script_path: PathBuf,

    /// Output file for the results; defaults to stdout if not specified.
    #[structopt(
        short = "o",
        long = "output",
        parse(from_os_str),
        help = "Output JSON file for results"
    )]
    pub output: Option<PathBuf>,

    /// Proxy server to use for all connections.
    #[structopt(short = "p", long = "proxy", help = "HTTP proxy server")]
    pub proxy: Option<String>,

    /// Limit for the number of requests.
    #[structopt(long = "requests-limit", default_value = "2000", help = "Request limit")]
    pub requests_limit: i32,

    /// Delay between requests in seconds.
    #[structopt(long = "delay", default_value = "5", help = "Delay between requests")]
    pub delay: u64,

    /// Path to the log file for debugging.
    #[structopt(long = "log", help = "Path to save logs for debugging")]
    pub log: Option<PathBuf>,

    /// Path to the file containing URLs to scan.
    #[structopt(
        long = "urls",
        parse(from_os_str),
        help = "File containing URLs to scan"
    )]
    pub urls: Option<PathBuf>,

    /// Custom headers as a JSON string.
    #[structopt(
        long = "headers",
        parse(try_from_str = parse_headers),
        required = false,
        default_value = "{}",
        help = "Custom HTTP headers in JSON format"
    )]
    pub headers: HeaderMap,

    /// Exit after a specified number of script errors.
    #[structopt(
        long = "exit-after-errors",
        default_value = "2000",
        help = "Exit after X number of script errors"
    )]
    pub exit_after: i32,

    /// Global environment variables for scripts.
    #[structopt(
        long = "env-vars",
        parse(try_from_str = get_env_vars),
        default_value = "{}",
        help = "Global environment variables for scripts in JSON format"
    )]
    pub env_vars: Value,

    /// Custom input handler script.
    #[structopt(
        long = "input-handler",
        parse(from_os_str),
        help = "Custom input handler script"
    )]
    pub input_handler: Option<PathBuf>,

    /// Resume scan using a configuration file from the previous run.
    #[structopt(
        long = "resume",
        parse(try_from_str = read_resume_file),
        help = "Resume scan using a resume configuration file"
    )]
    _resume: Option<()>,
}
