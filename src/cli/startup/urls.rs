use crate::cli::args::Opts;
use crate::cli::input::get_target_urls;
use crate::cli::logger::init_log;
use crate::show_msg;
use crate::CliErrors;
use crate::Lotus;
use crate::MessageLevel;
use crate::RequestOpts;
use std::sync::{Arc, Mutex};
use structopt::StructOpt;

pub struct UrlArgs {
    pub urls: Vec<String>,
    pub exit_after: i32,
    pub req_opts: RequestOpts,
    pub lotus_obj: Lotus,
}

pub fn args_urls() -> UrlArgs {
    let (urls, exit_after, req_opts, lotus_obj) = match Opts::from_args() {
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
            let urls = urls.unwrap();
            (urls, exit_after, req_opts, lotus_obj)
        }
        _ => {
            std::process::exit(1);
        }
    };

    UrlArgs {
        urls,
        exit_after,
        req_opts,
        lotus_obj,
    }
}
