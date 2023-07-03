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

use crate::cli::bar::{show_msg, MessageLevel};
use crate::ScanTypes;

lazy_static! {
    pub static ref LAST_HTTP_SCAN_ID: Arc<Mutex<usize>> = Arc::new(Mutex::new(0));
    pub static ref LAST_URL_SCAN_ID: Arc<Mutex<usize>> = Arc::new(Mutex::new(0));
    pub static ref LAST_HOST_SCAN_ID: Arc<Mutex<usize>> = Arc::new(Mutex::new(0));
    pub static ref LAST_PATH_SCAN_ID: Arc<Mutex<usize>> = Arc::new(Mutex::new(0));
    pub static ref LAST_CUSTOM_SCAN_ID: Arc<Mutex<usize>> = Arc::new(Mutex::new(0));
}

pub async fn pause_channel() {
    tokio::spawn(async move {
        ctrl_c().await.unwrap();
        if let Err(err) = generate_resume().await {
            show_msg(&err.to_string(), MessageLevel::Error)
        }
        /*
        match BROWSER_DRIVER.lock().await.close_window().await {
            Ok(..) => {}
            Err(err) => {
                show_msg(&err.to_string(), MessageLevel::Error);
            }
        }*/
        std::process::exit(130)
    });
}

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

    file.write_all(format!("HTTP_SCAN_ID={}\n", *http_scan_id).as_bytes())?;
    file.write_all(format!("URL_SCAN_ID={}\n", *url_scan_id).as_bytes())?;
    file.write_all(format!("HOST_SCAN_ID={}\n", *host_scan_id).as_bytes())?;
    file.write_all(format!("PATH_SCAN_ID={}\n", *path_scan_id).as_bytes())?;
    file.write_all(format!("CUSTOM_SCAN_ID={}\n", *custom_scan_id).as_bytes())?;

    Ok(())
}

async fn update_index_id(scan_type: Arc<ScanTypes>, index_id: usize) {
    match *scan_type {
        ScanTypes::FULL_HTTP => *LAST_HTTP_SCAN_ID.lock().await = index_id,
        ScanTypes::URLS => *LAST_URL_SCAN_ID.lock().await = index_id,
        ScanTypes::HOSTS => *LAST_HOST_SCAN_ID.lock().await = index_id,
        ScanTypes::PATHS => *LAST_PATH_SCAN_ID.lock().await = index_id,
        ScanTypes::CUSTOM => *LAST_CUSTOM_SCAN_ID.lock().await = index_id,
    }
}

// Asynchronous function to iterate over futures concurrently
// Takes a vector, a function and a number of workers as arguments
// The function must return a future with no output
// The vector must implement cloning
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

// This function takes a vector of futures, the number of worker threads to use, and an optional callback function
// It runs each future concurrently using the worker threads, and executes the callback function if provided
// The function exits once all futures have completed and the callback function has been executed (if provided)
pub async fn scan_futures<T: Future<Output = ()>>(
    scan_futures: Vec<T>,
    workers: usize,
    call_back: Option<fn()>,
) {
    // Create an unbounded MPSC (multiple producer, single consumer) channel
    // The producer sends futures to be executed and the consumer executes them concurrently
    let (mut sink, futures_stream) = mpsc::unbounded();

    // Create a read-write lock for the number of futures
    // This allows multiple threads to read the value while only one thread can write to it at a time
    let num_futures = RwLock::new(scan_futures.len());

    // Send all futures to the sink
    // This adds them to the stream of futures to be executed
    sink.send_all(&mut stream::iter(scan_futures.into_iter().map(Ok)))
        .await
        .unwrap();

    let sink_lock = RwLock::new(sink);

    // Execute each future concurrently using the specified number of worker threads
    // When a future completes, the callback function is executed (if provided)
    // Once all futures have completed, the sink is closed to exit the loop
    futures_stream
        .for_each_concurrent(workers, |fut| async {
            fut.await;

            // Decrement the number of futures and execute the callback function (if provided)
            let mut num_futures = num_futures.write().await;
            if let Some(..) = call_back {
                log::debug!("Running the Callback function for TASK {}", *num_futures);
                call_back.unwrap()();
                log::debug!("The Callback has been finished for TASK {}", *num_futures);
            }
            *num_futures -= 1;

            // If all futures have completed, close the sink to exit the loop
            if *num_futures == 0 {
                log::debug!("Running");
                sink_lock.write().await.close().await.unwrap();
            }
        })
        .await;
}
