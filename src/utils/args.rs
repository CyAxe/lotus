use regex::Regex;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "lotus", about = "A powerful CLI tool with advanced options.")]
pub struct Cli {
    #[structopt(short, long, default_value = "30", parse(try_from_str = parse_timeout))]
    pub timeout: u64,

    #[structopt(short, long, parse(try_from_str = parse_proxy))]
    pub proxy: Option<String>,

    #[structopt(short, long, parse(from_os_str))]
    pub output: PathBuf,
}

pub fn parse_timeout(s: &str) -> Result<u64, String> {
    s.parse::<u64>().map_err(|_| "Invalid timeout value. Must be a positive integer.".to_string())
}

pub fn parse_proxy(s: &str) -> Result<String, String> {
    let url_regex = Regex::new(r"^(http|https)://").map_err(|_| "Regex compilation failed.".to_string())?;
    if url_regex.is_match(s) {
        Ok(s.to_string())
    } else {
        Err("Invalid proxy URL. Must start with http:// or https://".to_string())
    }
}

impl Cli {
    pub fn validate(&self) -> Result<(), String> {
        if self.timeout == 0 {
            return Err("Timeout must be greater than zero.".to_string());
        }
        Ok(())
    }
}
