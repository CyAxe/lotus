use crate::cli::args::Opts;
use crate::cli::input::{get_target_hosts, get_target_paths, get_target_urls};
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
    pub delay: u64,
    pub fuzz_workers: usize,
    pub verbose: bool,
}

pub struct TargetData {
    pub urls: Vec<String>,
    pub hosts: Vec<String>,
    pub paths: Vec<String>,
}

pub fn args_urls() -> UrlArgs {
    let (
        urls,
        hosts,
        paths,
        exit_after,
        req_opts,
        lotus_obj,
        requests_limit,
        delay,
        fuzz_workers,
        verbose,
    ) = match Opts::from_args() {
        Opts::URLS(url_opts) => {
            // setup logger
            init_log(url_opts.log).unwrap();
            let req_opts = RequestOpts {
                headers: url_opts.headers,
                proxy: url_opts.proxy,
                timeout: url_opts.timeout,
                redirects: url_opts.redirects,
            };
            let lotus_obj = Lotus {
                script_path: url_opts.script_path,
                output: url_opts.output,
                workers: url_opts.workers,
                script_workers: url_opts.scripts_workers,
                stop_after: Arc::new(Mutex::new(1)),
            };
            let urls = get_target_urls(url_opts.urls);
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
            let paths = get_target_paths(urls_vec.clone());
            let hosts = get_target_hosts(urls_vec.clone());
            (
                urls_vec,
                hosts,
                paths,
                url_opts.exit_after,
                req_opts,
                lotus_obj,
                url_opts.requests_limit,
                url_opts.delay,
                url_opts.fuzz_workers,
                url_opts.verbose,
            )
        }
        _ => {
            std::process::exit(1);
        }
    };

    UrlArgs {
        target_data: TargetData { urls, hosts, paths },
        exit_after,
        req_opts,
        lotus_obj,
        requests_limit,
        delay,
        fuzz_workers,
        verbose,
    }
}
