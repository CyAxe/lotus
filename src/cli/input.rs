use crate::filename_to_string;
use crate::CliErrors;
use std::{io, io::BufRead, path::PathBuf};
use url::Url;
pub mod load_scripts;

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

pub fn get_target_paths(urls: Vec<String>) -> Vec<String> {
    let mut paths: Vec<String> = Vec::new();
    urls.iter().for_each(|x| {
        let the_path = Url::parse(x).unwrap().path().to_string();
        let new_url = Url::join(&Url::parse(x).unwrap(), &the_path);
        if new_url.is_ok(){
            let new_url = new_url.unwrap().to_string();
            if !paths.contains(&new_url) {
                paths.push(new_url);
            }
        } else {
            log::error!("Cannot URL Join {} with {}",x,&the_path);
            log::error!("UNWRAP ERROR {}",new_url.unwrap_err());
        }
    });
    paths
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
