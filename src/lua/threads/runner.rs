use futures::stream::{self, StreamExt};
use futures::{channel::mpsc, sink::SinkExt};
use futures::Future;
use tokio::sync::RwLock;


pub async fn scan_futures<T: Future<Output = ()>>(scan_futures: Vec<T>, workers: usize, call_back: Option<fn()>) {
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
            if call_back.is_some(){
                call_back.unwrap()()
            }
            *num_futures -= 1;
            if *num_futures <= 0 {
                // Close the sink to exit the for_each_concurrent
                sink_lock.write().await.close().await.unwrap();
            }
        })
        .await;
}
