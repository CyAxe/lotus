use crate::cli::errors::CliErrors;
use reqwest::header::HeaderMap;
use reqwest::header::{HeaderName, HeaderValue};
use std::collections::HashMap;
use std::path::PathBuf;
use serde_json::Value;
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

fn get_env_vars(env_vars_json: &str) -> Result<Value, serde_json::Error> {
    let parsed_vars = serde_json::from_str(env_vars_json)?;
    Ok(parsed_vars)
}

fn get_script_type(script_type: &str) -> Result<ScriptType, CliErrors> {
    let script_type = match script_type {
        "fuzz" => ScriptType::Fuzz,
        "cve" => ScriptType::CVE,
        "passive" => ScriptType::PASSIVE,
        "service" => ScriptType::SERVICE,
        _ => ScriptType::NotSupported,
    };
    if script_type == ScriptType::NotSupported {
        Err(CliErrors::UnsupportedScript)
    } else {
        Ok(script_type)
    }
}

#[derive(Debug, PartialEq)]
pub enum ScriptType {
    Fuzz,
    CVE,
    PASSIVE,
    SERVICE,
    NotSupported,
}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "Lotus",
    about = "Fast Web Security Scanner written in Rust based on Lua Scripts"
)]
pub enum Opts {
    #[structopt(about = "Create a lua example code based on the type of scan")]
    NEW {
        #[structopt(short = "-s", long, parse(try_from_str = get_script_type))]
        scan_type: ScriptType,
        #[structopt(short = "f", long)]
        file_name: PathBuf,
    },
    #[structopt(about = "Use CVE, VULN scripts to scan the given URLs")]
    SCAN {
        // redirects limit
        #[structopt(
            short,
            long,
            default_value = "10",
            help = "Number of allowed http redirects"
        )]
        redirects: u32,
        #[structopt(
            long = "fuzz-workers",
            default_value = "15",
            help = "The number of workers who will be involved in the fuzzing process"
        )]
        fuzz_workers: usize,

        // threads
        #[structopt(
            short = "w",
            long = "workers",
            default_value = "10",
            help = "Number of workers"
        )]
        workers: usize,
        #[structopt(
            short = "v",
            long = "verbose",
            help = "verbose mode (show sending requests)"
        )]
        verbose: bool,

        #[structopt(
            short = "sw",
            long = "scripts-worker",
            default_value = "10",
            help = "How many scripts to run at the same time for one url"
        )]
        scripts_workers: usize,

        // timeout
        #[structopt(
            short = "t",
            long = "timeout",
            default_value = "10",
            help = "Connection timeout"
        )]
        timeout: u64,

        /// Input file
        #[structopt(parse(from_os_str), help = "Scripts path")]
        script_path: PathBuf,

        /// Output file, stdout if not present
        #[structopt(
            short = "o",
            long = "output",
            parse(from_os_str),
            help = "output json file"
        )]
        output: Option<PathBuf>,

        #[structopt(
            short = "p",
            long = "proxy",
            help = "Set http proxy for all connections"
        )]
        proxy: Option<String>,
        #[structopt(
            long = "requests-limit",
            help = "requests limit",
            default_value = "2000"
        )]
        requests_limit: i32,
        #[structopt(long = "delay", help = "sleeping dalay", default_value = "5")]
        delay: u64,

        #[structopt(long = "log", help = "Saving Lotus Logs for debugging")]
        log: Option<PathBuf>,
        #[structopt(
            long = "urls",
            help = "Reading urls from text file",
            parse(from_os_str)
        )]
        urls: Option<PathBuf>,
        #[structopt(long = "requests", help = "Give input as full request in JSON")]
        is_request: bool,

        #[structopt(long = "headers", parse(try_from_str = parse_headers), required = false, default_value = "{}", help = "Default Headers (eg: '{\"X-API\":\"123\"}')")]
        headers: HeaderMap,
        #[structopt(
            long = "exit-after-errors",
            default_value = "2000",
            help = "Exit after X number of script errors"
        )]
        exit_after: i32,
        #[structopt(long = "env-vars",parse(try_from_str = get_env_vars),default_value="{}",help = "Set Global Vars for scripts")]
        env_vars: Value
    },
}
