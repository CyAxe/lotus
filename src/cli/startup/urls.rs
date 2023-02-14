use crate::cli::args::Opts;
use crate::cli::input::{get_target_hosts, get_target_urls};
use crate::cli::logger::init_log;
use crate::show_msg;
use crate::CliErrors;
use crate::Lotus;
use crate::MessageLevel;
use crate::RequestOpts;
use std::sync::{Arc, Mutex};
use structopt::StructOpt;

pub struct UrlArgs {
    pub target_data: TargetData,
    pub exit_after: i32,
    pub req_opts: RequestOpts,
    pub lotus_obj: Lotus,
    pub requests_limit: i32,
    pub delay: u64
}

pub struct TargetData {
    pub urls: Vec<String>,
    pub hosts: Vec<String>,
}

pub fn args_urls() -> UrlArgs {
    let (urls, hosts, exit_after, req_opts, lotus_obj, requests_limit, delay) = match Opts::from_args() {
        Opts::URLS {
            redirects,
            workers,
            scripts_workers,
            timeout,
            script_path,
            output,
            proxy,
            log,
            urls,
            headers,
            exit_after,
            requests_limit,
            delay
        } => {
            // setup logger
            init_log(log).unwrap();
            let req_opts = RequestOpts {
                headers,
                proxy,
                timeout,
                redirects,
            };
            let lotus_obj = Lotus {
                script_path,
                output,
                workers,
                script_workers: scripts_workers,
                stop_after: Arc::new(Mutex::new(1)),
            };
            let urls = get_target_urls(urls);
            if urls.is_err() {
                match urls {
                    Err(CliErrors::EmptyStdin) => {
                        show_msg("No input in Stdin", MessageLevel::Error);
                    }
                    Err(CliErrors::ReadingError) => {
                        show_msg("Cannot Read the urls file", MessageLevel::Error);
                    }
                    _ => {}
                };
                std::process::exit(1);
            }
            let urls_vec = urls
                .unwrap()
                .iter()
                .map(|url| url.to_string())
                .collect::<Vec<String>>();
            let hosts = get_target_hosts(urls_vec.clone());
            (urls_vec, hosts, exit_after, req_opts, lotus_obj, requests_limit, delay)
        }
        _ => {
            std::process::exit(1);
        }
    };

    UrlArgs {
        target_data: TargetData { urls, hosts },
        exit_after,
        req_opts,
        lotus_obj,
        requests_limit,
        delay
    }
}
