use futures::stream::{self, StreamExt};
use futures::Future;
use futures::{channel::mpsc, sink::SinkExt};
use tokio::sync::RwLock;

// Asynchronous function to iterate over futures concurrently
// Takes a vector, a function and a number of workers as arguments
// The function must return a future with no output
// The vector must implement cloning
pub async fn iter_futures<F, T, Fut>(target_iter: Vec<T>, target_function: F, workers: usize)
where
    F: FnOnce(T) -> Fut + Clone,
    Fut: Future<Output = ()>,
    T: Clone,
{
    stream::iter(target_iter)
        .for_each_concurrent(workers, |out| {
            let out = out.clone();
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
            if call_back.is_some() {
                log::debug!("Running the Callback function for TASK {}", *num_futures);
                call_back.unwrap()();
                log::debug!("The Callback has been finished for TASK {}", *num_futures);
            }
            *num_futures -= 1;

            // If all futures have completed, close the sink to exit the loop
            if *num_futures <= 0 {
                log::debug!("Running");
                sink_lock.write().await.close().await.unwrap();
            }
        })
        .await;
}
