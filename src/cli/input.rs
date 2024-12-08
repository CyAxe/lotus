use crate::filename_to_string;
use crate::CliErrors;
use std::{io, io::Read, path::PathBuf};
use url::Url;

pub mod load_scripts;
pub mod parse_requests;

/// Extracts unique target hosts from a list of URLs.
///
/// Ensures no duplicate or empty hosts are included.
pub fn get_target_hosts(urls: Vec<String>) -> Vec<String> {
    let mut hosts = Vec::new();
    urls.iter().for_each(|x| {
        let parsed_url = Url::parse(x);
        if let Ok(parsed_url) = parsed_url {
            let host = {
                if parsed_url.port().is_some() {
                    let host = parsed_url.host().unwrap();
                    let port = parsed_url.port().unwrap();
                    format!("{}:{}", host, port)
                } else {
                    let host = if let Some(host) = parsed_url.host() {
                        host
                    } else {
                        url::Host::Domain("") // Default to empty domain
                    };
                    host.to_string()
                }
            };
            if !hosts.contains(&host) && !host.is_empty() {
                hosts.push(host);
            }
        }
    });
    hosts.sort();
    hosts.dedup();
    hosts
}

/// Extracts unique target paths from a list of URLs.
///
/// Logs errors for invalid URLs and joins base URLs with their paths.
pub fn get_target_paths(urls: Vec<String>) -> Result<Vec<String>, String> {
    let mut paths: Vec<String> = Vec::new();
    for url_str in urls {
        let url = match Url::parse(&url_str) {
            Ok(url) => url,
            Err(err) => {
                log::error!("Failed to parse URL {}: {}", url_str, err);
                continue;
            }
        };
        let path = match url.path().to_string().as_str() {
            "" => "/".to_string(),
            path => path.to_string(),
        };
        let new_url = match url.join(&path) {
            Ok(new_url) => new_url,
            Err(err) => {
                log::error!("Failed to join URL {} with path {}: {}", url, path, err);
                continue;
            }
        };
        let new_url_str = new_url.to_string();
        if !paths.contains(&new_url_str) {
            paths.push(new_url_str);
        }
    }
    Ok(paths)
}

/// Reads input from stdin and returns it as a vector of strings.
///
/// Returns an error if stdin is empty or unavailable.
pub fn get_stdin_input() -> Result<Vec<String>, CliErrors> {
    if atty::is(atty::Stream::Stdin) {
        Err(CliErrors::EmptyStdin)
    } else {
        let stdin = io::stdin();
        let mut input_string = String::new();
        stdin.lock().read_to_string(&mut input_string).unwrap();
        let input_lines: Vec<String> = input_string.lines().map(|s| s.to_string()).collect();
        Ok(input_lines)
    }
}

/// Retrieves target URLs from a file or stdin.
///
/// Deduplicates and sorts the URLs if reading from stdin.
pub fn get_target_urls(url_file: Option<PathBuf>) -> Result<Vec<String>, CliErrors> {
    if url_file.is_some() {
        let urls = filename_to_string(url_file.unwrap().to_str().unwrap());
        if let Ok(urls) = urls {
            Ok(urls
                .lines()
                .map(|url| url.to_string())
                .collect::<Vec<String>>())
        } else {
            Err(CliErrors::ReadingError)
        }
    } else {
        let mut urls = get_stdin_input()?;
        urls.sort();
        urls.dedup();
        Ok(urls)
    }
}
