use reqwest::header::HeaderMap;
use reqwest::header::{HeaderName, HeaderValue};
use std::collections::HashMap;
use std::path::PathBuf;
use structopt::StructOpt;

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
    parsed_json
        .unwrap()
        .iter()
        .for_each(|(headername, headervalue)| {
            user_headers.insert(
                HeaderName::from_bytes(headername.as_bytes()).unwrap(),
                HeaderValue::from_bytes(headervalue.as_bytes()).unwrap(),
            );
        });
    Ok(user_headers)
}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "Lotus",
    about = "Fast Web Security Scanner written in Rust based on Lua Scripts"
)]
pub struct Opts {
    // redirects limit
    #[structopt(short, long, default_value = "10", help="Number of allowed http redirects")]
    pub redirects: u32,

    // threads
    #[structopt(short = "w", long = "workers", default_value = "10", help = "Number of workers")]
    pub workers: usize,

    #[structopt(short = "sw", long = "scripts-worker", default_value = "10", help = "How many scripts to run at the same time for one url")]
    pub scripts_workers: usize,

    // timeout
    #[structopt(short = "t", long = "timeout", default_value = "10", help = "Connection timeout")]
    pub timeout: u64,

    /// Input file
    #[structopt(parse(from_os_str), help = "Scripts path")]
    pub script_path: PathBuf,

    /// Output file, stdout if not present
    #[structopt(short = "o", long = "output", parse(from_os_str), help="output json file" )]
    pub output: Option<PathBuf>,

    #[structopt(short = "p", long = "proxy", help = "Set http proxy for all connections")]
    pub proxy: Option<String>,

    #[structopt(long = "log",help = "Saving Lotus Logs for debugging")]
    pub log: Option<PathBuf>,
    #[structopt(long = "urls", help = "Reading urls from text file", parse(from_os_str))]
    pub urls: Option<PathBuf>,

    #[structopt(long = "headers", parse(try_from_str = parse_headers), required = false, default_value = "{}", help = "Default Headers (eg: '{\"X-API\":\"123\"}')")]
    pub headers: HeaderMap,
    #[structopt(long = "exit-after-errors", default_value = "2000", help = "Exit after X number of script errors")]
    pub exit_after: i32,
}
