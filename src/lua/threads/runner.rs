// This file is part of Lotus Project, a web security scanner written in Rust based on Lua scripts.
// Handles asynchronous task execution and provides utilities for managing scan progress.

use futures::executor::block_on;
use futures::stream::{self, StreamExt};
use futures::Future;
use futures::{channel::mpsc, sink::SinkExt};
use lazy_static::lazy_static;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::sync::Arc;
use tokio::signal::ctrl_c;
use tokio::sync::Mutex;
use tokio::sync::RwLock;

use crate::ScanTypes;

lazy_static! {
    // Shared mutable counters for tracking scan progress across different scan types.
    pub static ref LAST_HTTP_SCAN_ID: Arc<Mutex<usize>> = Arc::new(Mutex::new(0));
    pub static ref LAST_URL_SCAN_ID: Arc<Mutex<usize>> = Arc::new(Mutex::new(0));
    pub static ref LAST_HOST_SCAN_ID: Arc<Mutex<usize>> = Arc::new(Mutex::new(0));
    pub static ref LAST_PATH_SCAN_ID: Arc<Mutex<usize>> = Arc::new(Mutex::new(0));
    pub static ref LAST_CUSTOM_SCAN_ID: Arc<Mutex<usize>> = Arc::new(Mutex::new(0));
}

// Spawns a thread to handle application termination via CTRL+C.
// Generates a resume file upon termination to save scan progress.
pub async fn pause_channel() {
    tokio::spawn(async move {
        ctrl_c().await.unwrap();
        if let Err(err) = generate_resume().await {
            log::error!("Error generating resume file: {}", err);
        }
        std::process::exit(130)
    });
}

// Writes the current scan progress to a resume configuration file.
async fn generate_resume() -> Result<(), std::io::Error> {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open("resume.cfg")?;

    let http_scan_id = LAST_HTTP_SCAN_ID.lock().await;
    let url_scan_id = LAST_URL_SCAN_ID.lock().await;
    let host_scan_id = LAST_HOST_SCAN_ID.lock().await;
    let path_scan_id = LAST_PATH_SCAN_ID.lock().await;
    let custom_scan_id = LAST_CUSTOM_SCAN_ID.lock().await;

    file.write_all(format!("HTTP_SCAN_ID={}
", *http_scan_id).as_bytes())?;
    file.write_all(format!("URL_SCAN_ID={}
", *url_scan_id).as_bytes())?;
    file.write_all(format!("HOST_SCAN_ID={}
", *host_scan_id).as_bytes())?;
    file.write_all(format!("PATH_SCAN_ID={}
", *path_scan_id).as_bytes())?;
    file.write_all(format!("CUSTOM_SCAN_ID={}
", *custom_scan_id).as_bytes())?;

    Ok(())
}

// Updates the scan progress ID for the specified scan type.
async fn update_index_id(scan_type: Arc<ScanTypes>, index_id: usize) {
    match *scan_type {
        ScanTypes::FULL_HTTP => *LAST_HTTP_SCAN_ID.lock().await = index_id,
        ScanTypes::URLS => *LAST_URL_SCAN_ID.lock().await = index_id,
        ScanTypes::HOSTS => *LAST_HOST_SCAN_ID.lock().await = index_id,
        ScanTypes::PATHS => *LAST_PATH_SCAN_ID.lock().await = index_id,
        ScanTypes::CUSTOM => *LAST_CUSTOM_SCAN_ID.lock().await = index_id,
    }
}

// Iterates over a list of items and applies a processing function concurrently.
// Supports progress tracking and optional index updates.
pub async fn iter_futures<F, T, Fut>(
    scan_type: Arc<ScanTypes>,
    target_iter: Vec<T>,
    target_function: F,
    workers: usize,
    skip_index: usize,
    count_index: bool,
) where
    F: FnOnce(T) -> Fut + Clone,
    Fut: Future<Output = ()>,
    T: Clone,
{
    stream::iter(target_iter)
        .enumerate()
        .skip(skip_index)
        .for_each_concurrent(workers, |(index_id, out)| {
            let scan_type = Arc::clone(&scan_type);
            if count_index {
                block_on(update_index_id(scan_type, index_id));
            }
            let target_function = target_function.clone();
            async move { target_function(out).await }
        })
        .await;
}

// Executes a list of asynchronous tasks with a configurable number of workers.
// Optionally runs a callback function after each task completion.
pub async fn scan_futures<T: Future<Output = ()>>(
    scan_futures: Vec<T>,
    workers: usize,
    call_back: Option<fn()>,
) {
    let (mut sink, futures_stream) = mpsc::unbounded();
    let num_futures = RwLock::new(scan_futures.len());

    sink.send_all(&mut stream::iter(scan_futures.into_iter().map(Ok)))
        .await
        .unwrap();

    let sink_lock = RwLock::new(sink);

    futures_stream
        .for_each_concurrent(workers, |fut| async {
            fut.await;

            let mut num_futures = num_futures.write().await;
            if let Some(..) = call_back {
                log::debug!("Running callback for task {}", *num_futures);
                call_back.unwrap()();
            }
            *num_futures -= 1;

            if *num_futures == 0 {
                log::debug!("All tasks completed. Closing stream.");
                sink_lock.write().await.close().await.unwrap();
            }
        })
        .await;
}
