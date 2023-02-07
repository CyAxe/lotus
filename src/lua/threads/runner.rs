use futures::stream::{self, StreamExt};
use futures::Future;
use futures::{channel::mpsc, sink::SinkExt};
use tokio::sync::RwLock;

pub async fn iter_futures_tuple<F, Fut>(target_iter: Vec<(String, String)>, target_function: F, workers: usize) where F: FnOnce((String, String)) -> Fut + Clone ,
Fut: Future<Output = ()> 
{
    stream::iter(target_iter)
    .for_each_concurrent(workers, |out| { 
        let out = out.clone(); 
        let target_function = target_function.clone();
        async move {target_function(out).await}}).await;
}

pub async fn iter_futures<F, Fut>(target_iter: Vec<String>, target_function: F, workers: usize) where F: FnOnce(String) -> Fut + Clone ,
Fut: Future<Output = ()> 
{
    stream::iter(target_iter)
    .for_each_concurrent(workers, |out| { 
        let out = out.clone(); 
        let target_function = target_function.clone();
        async move {target_function(out).await}}).await;
}

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
            if call_back.is_some() {
                log::debug!("Running the Callback function for TASK {}", *num_futures);
                call_back.unwrap()();
                log::debug!("The Callback has been finished for TASK {}", *num_futures);
            }
            *num_futures -= 1;
            if *num_futures <= 0 {
                // Close the sink to exit the for_each_concurrent
                log::debug!("Running");
                sink_lock.write().await.close().await.unwrap();
            }
        })
        .await;
}
