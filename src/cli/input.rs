use crate::filename_to_string;
use crate::CliErrors;
use std::{io, io::BufRead, path::PathBuf};
use url::Url;
pub mod load_scripts;
pub mod parse_requests;

pub fn get_target_hosts(urls: Vec<String>) -> Vec<String> {
    let mut hosts = Vec::new();
    urls.iter().for_each(|x| {
        let parsed_url = Url::parse(x);
        if parsed_url.is_ok() {
            let parsed_url = parsed_url.unwrap();
            let host = {
                let host = parsed_url.host().unwrap();
                if parsed_url.port().is_some() {
                    let port = parsed_url.port().unwrap();
                    format!("{}:{}", host, port)
                } else {
                    host.to_string()
                }
            };
            if !hosts.contains(&host) {
                hosts.push(host);
            }
        }
    });
    hosts.sort();
    hosts.dedup();
    hosts
}

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

pub fn get_target_urls(url_file: Option<PathBuf>) -> Result<Vec<String>, CliErrors> {
    if url_file.is_some() {
        let urls = filename_to_string(url_file.unwrap().to_str().unwrap());
        if urls.is_ok() {
            Ok(urls
                .unwrap()
                .lines()
                .map(|url| url.to_string())
                .collect::<Vec<String>>())
        } else {
            Err(CliErrors::ReadingError)
        }
    } else {
        if atty::is(atty::Stream::Stdin) {
            Err(CliErrors::EmptyStdin)
        } else {
            let stdin = io::stdin();
            let mut urls: Vec<String> = Vec::new();
            stdin.lock().lines().for_each(|x| {
                let the_url = x.unwrap();
                match url::Url::parse(&the_url) {
                    Ok(..) => {
                        urls.push(the_url);
                    }
                    Err(..) => {
                        log::error!("Cannot Parse {} url, ignoring ..", the_url);
                    }
                };
            });
            urls.sort();
            urls.dedup();
            Ok(urls)
        }
    }
}
