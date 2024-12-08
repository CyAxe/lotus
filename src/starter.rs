use lotus::{
    cli::{
        input::load_scripts::get_scripts,
        startup::scan::scan::args_scan,
    },
    lua::{
        network::http::{REQUESTS_LIMIT, SLEEP_TIME, VERBOSE_MODE},
        threads::runner,
    },
    utils::{
        bar::{ProgressManager, GLOBAL_PROGRESS_BAR},
        logger::init_logger,
    },
    ScanTypes,
};

pub async fn run_scan() -> Result<(), std::io::Error> {
    // Spawn a new thread to handle the exit process when the user presses CTRL + C.
    runner::pause_channel().await;
    let opts = args_scan();
    let scripts = get_scripts(opts.lotus_obj.script_path.clone());
    let fuzz_workers = opts.fuzz_workers;
    log::info!(
        "{}",
        &format!("Number of URLs: {}", opts.target_data.urls.len()),
    );

    log::info!(
        "{}",
        &format!("Number of hosts: {}", opts.target_data.hosts.len()),
    );

    log::info!(
        "{}",
        &format!(
            "Number of HTTP MSGS: {}",
            opts.target_data.parse_requests.len()
        ),
    );
    log::info!(
        "{}",
        &format!("Number of paths: {}", opts.target_data.paths.len()),
    );

    log::info!(
        "{}",
        &format!(
            "Number of custom entries: {}",
            opts.target_data.custom.len()
        ),
    );
    // Open two threads for URL/HOST scanning
    let prog = ProgressManager::new(
        (opts.target_data.hosts.len()
            + opts.target_data.paths.len()
            + opts.target_data.custom.len()
            + opts.target_data.urls.len() * scripts.len()) as u64,
        "",
    );
    init_logger(prog.progress_bar);
    {
        *SLEEP_TIME.lock().await = opts.delay;
        *REQUESTS_LIMIT.lock().await = opts.requests_limit;
        *VERBOSE_MODE.lock().await = opts.verbose;
    }
    let scan_futures = vec![
        opts.lotus_obj.start(
            opts.target_data.parse_requests,
            scripts.clone(),
            opts.req_opts.clone(),
            ScanTypes::FULL_HTTP,
            opts.exit_after,
            fuzz_workers,
        ),
        opts.lotus_obj.start(
            convert_serde_value(opts.target_data.paths),
            scripts.clone(),
            opts.req_opts.clone(),
            ScanTypes::PATHS,
            opts.exit_after,
            fuzz_workers,
        ),
        opts.lotus_obj.start(
            convert_serde_value(opts.target_data.urls),
            scripts.clone(),
            opts.req_opts.clone(),
            ScanTypes::URLS,
            opts.exit_after,
            fuzz_workers,
        ),
        opts.lotus_obj.start(
            convert_serde_value(opts.target_data.hosts),
            scripts.clone(),
            opts.req_opts.clone(),
            ScanTypes::HOSTS,
            opts.exit_after,
            fuzz_workers,
        ),
        opts.lotus_obj.start(
            opts.target_data.custom,
            scripts.clone(),
            opts.req_opts,
            ScanTypes::CUSTOM,
            opts.exit_after,
            fuzz_workers,
        ),
    ];
    runner::scan_futures(scan_futures, 4, None).await;
    GLOBAL_PROGRESS_BAR
        .lock()
        .unwrap()
        .clone()
        .unwrap()
        .finish_and_clear();
    Ok(())
}

pub fn convert_serde_value(data: Vec<String>) -> Vec<serde_json::Value> {
    data.into_iter()
        .map(|s| serde_json::Value::String(s))
        .collect()
}
