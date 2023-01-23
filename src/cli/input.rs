use crate::filename_to_string;
use crate::CliErrors;
use std::{io, io::BufRead, path::PathBuf};

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
            Ok(urls)
        }
    }
}
