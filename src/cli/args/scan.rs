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
    /// The maximum number of HTTP redirects allowed during scanning.
    #[structopt(short, long, default_value = "10", help = "Set the maximum number of HTTP redirects")]
    pub redirects: u32,

    /// Number of concurrent fuzzing workers to use.
    #[structopt(long = "fuzz-workers", default_value = "15", help = "Specify the number of fuzzing workers")]
    pub fuzz_workers: usize,

    /// Types of content to scan, specified as a comma-separated list.
    #[structopt(
        short,
        long = "content-type",
        default_value = "url,body,headers",
        parse(try_from_str = parse_scan_content_type),
        help = "Set content types for scanning (e.g., url, body, headers)"
    )]
    pub _content_type: (),

    /// Number of worker threads for concurrent processing.
    #[structopt(short = "w", long = "workers", default_value = "10", help = "Set the number of worker threads")]
    pub workers: usize,

    /// Enable verbose mode to display detailed request information.
    #[structopt(short = "v", long = "verbose", help = "Enable detailed output for debugging")]
    pub verbose: bool,

    /// Maximum number of scripts to execute concurrently per URL.
    #[structopt(
        short = "sw",
        long = "scripts-worker",
        default_value = "10",
        help = "Specify the number of concurrent scripts per URL"
    )]
    pub scripts_workers: usize,

    /// Timeout for connections, specified in seconds.
    #[structopt(short = "t", long = "timeout", default_value = "10", help = "Set the connection timeout in seconds")]
    pub timeout: u64,

    /// Path to the directory containing scan scripts.
    #[structopt(parse(from_os_str), help = "Specify the directory containing scan scripts")]
    pub script_path: PathBuf,

    /// Output file for storing scan results; defaults to stdout if not specified.
    #[structopt(
        short = "o",
        long = "output",
        parse(from_os_str),
        help = "Specify a file to save scan results in JSON format"
    )]
    pub output: Option<PathBuf>,

    /// Proxy server to route all HTTP connections through.
    #[structopt(short = "p", long = "proxy", help = "Set an HTTP proxy server for connections")]
    pub proxy: Option<String>,

    /// Limit the total number of requests sent during the scan.
    #[structopt(long = "requests-limit", default_value = "2000", help = "Set a limit for the total number of requests")]
    pub requests_limit: i32,

    /// Delay between consecutive requests, specified in seconds.
    #[structopt(long = "delay", default_value = "5", help = "Set a delay between each request in seconds")]
    pub delay: u64,

    /// File to log detailed debugging information.
    #[structopt(long = "log", help = "Specify a log file for debugging information")]
    pub log: Option<PathBuf>,

    /// Path to a file containing a list of URLs to scan.
    #[structopt(
        long = "urls",
        parse(from_os_str),
        help = "Provide a file with a list of URLs to scan"
    )]
    pub urls: Option<PathBuf>,

    /// Custom HTTP headers in JSON format.
    #[structopt(
        long = "headers",
        parse(try_from_str = parse_headers),
        required = false,
        default_value = "{}",
        help = "Set custom HTTP headers in JSON format"
    )]
    pub headers: HeaderMap,

    /// Exit the scan after encountering a specified number of errors.
    #[structopt(
        long = "exit-after-errors",
        default_value = "2000",
        help = "Stop the scan after a specified number of errors"
    )]
    pub exit_after: i32,

    /// Global environment variables for the scan scripts in JSON format.
    #[structopt(
        long = "env-vars",
        parse(try_from_str = get_env_vars),
        default_value = "{}",
        help = "Define global environment variables for scripts in JSON format"
    )]
    pub env_vars: Value,

    /// Custom Lua script for handling input.
    #[structopt(
        long = "input-handler",
        parse(from_os_str),
        help = "Specify a custom Lua script to handle input processing"
    )]
    pub input_handler: Option<PathBuf>,

    /// Resume the scan using a previously generated configuration file.
    #[structopt(
        long = "resume",
        parse(try_from_str = read_resume_file),
        help = "Resume the scan using a configuration file from a previous run"
    )]
    _resume: Option<()>,
}
