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

/// Runs the main scanning process asynchronously.
/// This function orchestrates the scan by initializing configurations,
/// spawning concurrent tasks, and managing progress reporting.
pub async fn run_scan() -> Result<(), std::io::Error> {
    // Initialize a thread to handle the exit process on CTRL + C signal.
    runner::pause_channel().await;

    // Parse command-line arguments for scanning configurations.
    let opts = args_scan();

    // Load Lua scripts from the specified path for scanning logic.
    let scripts = get_scripts(opts.lotus_obj.script_path.clone());

    // Set the number of workers for fuzzing operations.
    let fuzz_workers = opts.fuzz_workers;

    // Log details about the target data for debugging and transparency.
    log::info!("{}", &format!("Number of URLs: {}", opts.target_data.urls.len()));
    log::info!("{}", &format!("Number of hosts: {}", opts.target_data.hosts.len()));
    log::info!("{}", &format!("Number of HTTP MSGS: {}", opts.target_data.parse_requests.len()));
    log::info!("{}", &format!("Number of paths: {}", opts.target_data.paths.len()));
    log::info!("{}", &format!("Number of custom entries: {}", opts.target_data.custom.len()));

    // Initialize a progress manager to track scan progress across multiple threads.
    let prog = ProgressManager::new(
        (opts.target_data.hosts.len()
            + opts.target_data.paths.len()
            + opts.target_data.custom.len()
            + opts.target_data.urls.len() * scripts.len()) as u64,
        "",
    );

    // Set up the logger to integrate with the progress bar.
    init_logger(prog.progress_bar);

    // Configure global parameters for request handling.
    {
        *SLEEP_TIME.lock().await = opts.delay; // Delay between requests.
        *REQUESTS_LIMIT.lock().await = opts.requests_limit; // Maximum concurrent requests.
        *VERBOSE_MODE.lock().await = opts.verbose; // Enable verbose output.
    }

    // Define scanning tasks for different target types (URLs, Hosts, Paths, etc.).
    let scan_futures = vec![
        opts.lotus_obj.start(
            opts.target_data.parse_requests, // Parsed HTTP requests.
            scripts.clone(),
            opts.req_opts.clone(),
            ScanTypes::FULL_HTTP, // Full HTTP scan type.
            opts.exit_after,
            fuzz_workers,
        ),
        opts.lotus_obj.start(
            convert_serde_value(opts.target_data.paths), // Convert paths to serde values.
            scripts.clone(),
            opts.req_opts.clone(),
            ScanTypes::PATHS, // Path scan type.
            opts.exit_after,
            fuzz_workers,
        ),
        opts.lotus_obj.start(
            convert_serde_value(opts.target_data.urls),
            scripts.clone(),
            opts.req_opts.clone(),
            ScanTypes::URLS, // URL scan type.
            opts.exit_after,
            fuzz_workers,
        ),
        opts.lotus_obj.start(
            convert_serde_value(opts.target_data.hosts),
            scripts.clone(),
            opts.req_opts.clone(),
            ScanTypes::HOSTS, // Host scan type.
            opts.exit_after,
            fuzz_workers,
        ),
        opts.lotus_obj.start(
            opts.target_data.custom, // Custom-defined targets.
            scripts.clone(),
            opts.req_opts,
            ScanTypes::CUSTOM, // Custom scan type.
            opts.exit_after,
            fuzz_workers,
        ),
    ];

    // Execute scanning tasks concurrently with a thread limit for efficiency.
    runner::scan_futures(scan_futures, 4, None).await;

    // Finalize and clear the global progress bar after completing all tasks.
    GLOBAL_PROGRESS_BAR
        .lock()
        .unwrap()
        .clone()
        .unwrap()
        .finish_and_clear();

    // Return success result.
    Ok(())
}

/// Converts a vector of strings into a vector of serde_json::Value.
/// This utility function standardizes input data for scanning tasks.
///
/// # Arguments
/// * `data` - A vector of strings to be converted.
///
/// # Returns
/// * `Vec<serde_json::Value>` - A vector of serde_json::Value containing string data.
pub fn convert_serde_value(data: Vec<String>) -> Vec<serde_json::Value> {
    data.into_iter()
        .map(|s| serde_json::Value::String(s))
        .collect()
}
