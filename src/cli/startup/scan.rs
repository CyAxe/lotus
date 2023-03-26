use crate::cli::args::Opts;
use crate::cli::input::{get_target_hosts, get_target_paths, get_target_urls};
use crate::cli::logger::init_log;
use crate::CliErrors;
use crate::Lotus;
use crate::RequestOpts;
use crate::{show_msg, MessageLevel};
use std::sync::{Arc, Mutex};
use structopt::StructOpt;

pub struct ScanArgs {
    pub target_data: TargetData,
    pub exit_after: i32,
    pub is_request: bool,
    pub req_opts: RequestOpts,
    pub lotus_obj: Lotus,
    pub requests_limit: i32,
    pub delay: u64,
    pub fuzz_workers: usize,
    pub verbose: bool,
}

pub struct TargetData {
    pub urls: Vec<String>,
    pub hosts: Vec<String>,
    pub paths: Vec<String>,
}

pub fn args_scan() -> ScanArgs {
    let (
        urls,
        hosts,
        paths,
        exit_after,
        is_request,
        req_opts,
        lotus_obj,
        requests_limit,
        delay,
        fuzz_workers,
        verbose,
    ) = match Opts::from_args() {
        Opts::SCAN {
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
            delay,
            fuzz_workers,
            verbose,
            is_request,
            env_vars
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
                env_vars
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
            let paths = match get_target_paths(urls_vec.clone()) {
                Ok(paths) => paths,
                Err(err) => {
                    show_msg(
                        &format!("Failed to get target paths: {}", err),
                        MessageLevel::Error,
                    );
                    vec![]
                }
            };
            let hosts = get_target_hosts(urls_vec.clone());
            (
                urls_vec,
                hosts,
                paths,
                exit_after,
                is_request,
                req_opts,
                lotus_obj,
                requests_limit,
                delay,
                fuzz_workers,
                verbose,
            )
        }
        _ => {
            std::process::exit(1);
        }
    };

    ScanArgs {
        target_data: TargetData { urls, hosts, paths },
        exit_after,
        is_request,
        req_opts,
        lotus_obj,
        requests_limit,
        delay,
        fuzz_workers,
        verbose,
    }
}
